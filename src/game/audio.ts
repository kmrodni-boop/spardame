let ctx: AudioContext | null = null;
let master: GainNode | null = null;
let unlocked = false;
let noise: AudioBuffer | null = null;

function getCtx(): AudioContext | null {
  if (typeof window === "undefined") return null;
  if (!ctx) {
    const AC = window.AudioContext || (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext;
    if (!AC) return null;
    ctx = new AC({ latencyHint: "interactive" });
    master = ctx.createGain();
    master.gain.value = 0.75;
    master.connect(ctx.destination);
  }
  return ctx;
}

export function unlockAudio(): void {
  const audio = getCtx();
  if (!audio || !master) return;
  if (audio.state === "suspended") void audio.resume();
  unlocked = true;
}

function noiseBuffer(audio: AudioContext): AudioBuffer {
  if (noise && noise.sampleRate === audio.sampleRate) return noise;
  const length = Math.floor(audio.sampleRate * 0.5);
  const buf = audio.createBuffer(1, length, audio.sampleRate);
  const data = buf.getChannelData(0);
  for (let i = 0; i < length; i++) data[i] = Math.random() * 2 - 1;
  noise = buf;
  return buf;
}

type Burst = {
  when?: number;
  duration: number;
  gain: number;
  freq: number;
  q?: number;
  filter?: BiquadFilterType;
  rate?: number;
};

function burst(opts: Burst): void {
  const audio = getCtx();
  if (!audio || !master || !unlocked) return;
  const t = audio.currentTime + (opts.when ?? 0);
  const src = audio.createBufferSource();
  src.buffer = noiseBuffer(audio);
  src.playbackRate.value = opts.rate ?? 1;
  const filter = audio.createBiquadFilter();
  filter.type = opts.filter ?? "bandpass";
  filter.frequency.setValueAtTime(opts.freq, t);
  filter.Q.value = opts.q ?? 1;
  const g = audio.createGain();
  g.gain.setValueAtTime(0.0001, t);
  g.gain.exponentialRampToValueAtTime(Math.max(opts.gain, 0.0002), t + 0.005);
  g.gain.exponentialRampToValueAtTime(Math.max(opts.gain * 0.38, 0.0002), t + opts.duration * 0.42);
  g.gain.exponentialRampToValueAtTime(0.0001, t + opts.duration);
  src.connect(filter);
  filter.connect(g);
  g.connect(master);
  src.start(t);
  src.stop(t + opts.duration + 0.05);
  src.onended = () => {
    src.disconnect();
    filter.disconnect();
    g.disconnect();
  };
}

function beep(freq: number, dur: number, type: OscillatorType, gain = 0.07, slide?: number): void {
  const audio = getCtx();
  if (!audio || !master || !unlocked) return;
  const osc = audio.createOscillator();
  const g = audio.createGain();
  osc.type = type;
  osc.frequency.setValueAtTime(freq, audio.currentTime);
  if (slide) osc.frequency.exponentialRampToValueAtTime(slide, audio.currentTime + dur);
  g.gain.setValueAtTime(0.0001, audio.currentTime);
  g.gain.exponentialRampToValueAtTime(gain, audio.currentTime + 0.012);
  g.gain.exponentialRampToValueAtTime(0.0001, audio.currentTime + dur);
  osc.connect(g);
  g.connect(master);
  osc.start();
  osc.stop(audio.currentTime + dur + 0.02);
}

/** Snap of a playing card hitting felt — noise layers, no pitched beep. */
export function sfxCard(): void {
  const flick = 1800 + Math.random() * 1100;
  const paper = 720 + Math.random() * 240;
  const rate = 0.88 + Math.random() * 0.2;
  burst({ duration: 0.055, gain: 0.2, freq: flick, q: 0.85, filter: "bandpass", rate });
  burst({ duration: 0.13, gain: 0.13, freq: paper, q: 0.5, filter: "bandpass", rate });
  burst({ duration: 0.2, gain: 0.11, freq: 150 + Math.random() * 40, q: 0.5, filter: "lowpass" });
  burst({ duration: 0.24, gain: 0.05, freq: 420 + Math.random() * 80, q: 0.4, filter: "lowpass" });
}

export function sfxTrick(): void {
  sfxCard();
  burst({ when: 0.05, duration: 0.05, gain: 0.12, freq: 1450 + Math.random() * 400, q: 0.8, filter: "bandpass" });
  burst({ when: 0.09, duration: 0.14, gain: 0.13, freq: 200, q: 0.42, filter: "lowpass" });
}

export function sfxMoon(): void {
  beep(392, 0.16, "sine", 0.05);
  setTimeout(() => beep(523, 0.2, "sine", 0.045), 90);
  setTimeout(() => beep(659, 0.28, "sine", 0.04), 180);
}

export function sfxIllegal(): void {
  burst({ duration: 0.11, gain: 0.14, freq: 170, q: 0.7, filter: "lowpass" });
}

export function sfxPass(): void {
  burst({ duration: 0.028, gain: 0.14, freq: 2200, q: 0.9, filter: "bandpass", rate: 1.05 });
  burst({ when: 0.038, duration: 0.032, gain: 0.12, freq: 1750, q: 0.8, filter: "bandpass", rate: 0.95 });
  burst({ when: 0.078, duration: 0.036, gain: 0.1, freq: 2400, q: 1, filter: "bandpass", rate: 1.08 });
}

export function resumeAudio(): void {
  if (!ctx) return;
  if (ctx.state === "suspended") void ctx.resume();
}
