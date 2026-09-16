let ctx: AudioContext | null = null;
let master: GainNode | null = null;
let unlocked = false;

function getCtx(): AudioContext | null {
  if (typeof window === "undefined") return null;
  if (!ctx) {
    const AC = window.AudioContext || (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext;
    if (!AC) return null;
    ctx = new AC({ latencyHint: "interactive" });
    master = ctx.createGain();
    master.gain.value = 0.7;
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

export function sfxCard(): void {
  beep(420 + Math.random() * 40, 0.07, "triangle", 0.045);
}

export function sfxTrick(): void {
  beep(240, 0.14, "sine", 0.05, 180);
}

export function sfxMoon(): void {
  beep(392, 0.16, "sine", 0.06);
  setTimeout(() => beep(523, 0.2, "sine", 0.05), 90);
  setTimeout(() => beep(659, 0.28, "sine", 0.05), 180);
}

export function sfxIllegal(): void {
  beep(110, 0.12, "square", 0.03);
}

export function sfxPass(): void {
  beep(520, 0.08, "triangle", 0.04, 640);
}

export function resumeAudio(): void {
  if (!ctx) return;
  if (ctx.state === "suspended") void ctx.resume();
}
