//! Short card-play sound. Writes a tiny WAV once, then plays it with
//! PipeWire / Pulse / ALSA — whatever is on the machine.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;

const SAMPLE_RATE: u32 = 22050;
const DURATION_MS: u32 = 240;

fn cache_wav() -> PathBuf {
    static PATH: OnceLock<PathBuf> = OnceLock::new();
    PATH.get_or_init(|| {
        let mut dir = std::env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|h| {
                    let mut p = PathBuf::from(h);
                    p.push(".cache");
                    p
                })
            })
            .unwrap_or_else(|| PathBuf::from("/tmp"));
        dir.push("spardame");
        let _ = std::fs::create_dir_all(&dir);
        dir.push("card-play.wav");
        dir
    })
    .clone()
}

fn write_wav(path: &PathBuf) {
    let n = (SAMPLE_RATE * DURATION_MS / 1000) as usize;
    let mut pcm: Vec<i16> = Vec::with_capacity(n);
    let mut seed: u32 = 0xC0FFEE;
    for i in 0..n {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((seed >> 16) as i16 as f32) / 32768.0;
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = (-t * 14.0).exp();
        let paper = ((t * 780.0 * std::f32::consts::TAU).sin() * 0.25
            + (t * 190.0 * std::f32::consts::TAU).sin() * 0.18)
            * env;
        let thud = (t * 150.0 * std::f32::consts::TAU).sin() * (-t * 8.0).exp() * 0.22;
        let sample = (noise * 0.28 * env + paper + thud).clamp(-1.0, 1.0);
        pcm.push((sample * 18000.0) as i16);
    }

    let data_bytes = pcm.len() * 2;
    let mut buf = Vec::with_capacity(44 + data_bytes);
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&(36 + data_bytes as u32).to_le_bytes());
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    buf.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes());
    buf.extend_from_slice(&2u16.to_le_bytes());
    buf.extend_from_slice(&16u16.to_le_bytes());
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&(data_bytes as u32).to_le_bytes());
    for s in pcm {
        buf.extend_from_slice(&s.to_le_bytes());
    }
    if let Ok(mut f) = std::fs::File::create(path) {
        let _ = f.write_all(&buf);
    }
}

pub fn sfx_card() {
    std::thread::spawn(|| {
        let path = cache_wav();
        if !path.exists() || path.metadata().map(|m| m.len() < 80).unwrap_or(true) {
            write_wav(&path);
        }
        let p = path.to_string_lossy().to_string();
        let attempts: [(&str, &[&str]); 3] = [
            ("pw-play", &[]),
            ("paplay", &[]),
            ("aplay", &["-q"]),
        ];
        for (bin, extra) in attempts {
            let mut cmd = Command::new(bin);
            cmd.args(extra).arg(&p);
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
            if cmd.status().map(|s| s.success()).unwrap_or(false) {
                return;
            }
        }
    });
}

pub fn sfx_trick() {
    sfx_card();
}
