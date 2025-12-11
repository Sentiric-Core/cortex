/*
 * SENTIRIC CORTEX ENGINE
 * ----------------------
 * CORE LOOP v1.2 - First Words
 */

mod audio;
mod cognition;

use std::time::{Instant, Duration};
use std::thread;
use crate::audio::Ear;
use crate::cognition::Brain;

struct EngineState {
    is_running: bool,
    ticks: u64,
}

fn main() -> anyhow::Result<()> {
    println!("-------------------------------------------");
    println!("SENTIRIC CORTEX ENGINE: NEURAL LINK");
    println!("-------------------------------------------");

    let mut state = EngineState { is_running: true, ticks: 0 };

    // 1. Modülleri Başlat
    let ear = Ear::new()?;
    let brain = Brain::new()?;

    // Ses Tamponu (Buffer): Kesintisiz konuşmayı biriktirmek için
    // Whisper'a parça parça göndermek yerine, 3 saniyelik bloklar halinde göndereceğiz.
    let mut audio_buffer: Vec<f32> = Vec::new();
    let flush_threshold = 16000 * 3; // 3 saniye (16kHz)

    println!("[SYSTEM] Motor hazır. Konuşun... (Her 3 saniyede bir analiz edilir)");

    while state.is_running {
        state.ticks += 1;
        
        // --- 1. SENSE (İşitme) ---
        if let Some(packet) = ear.listen() {
            audio_buffer.extend_from_slice(&packet.samples);
        }

        // --- 2. PERCEIVE (Algılama Döngüsü) ---
        // Şimdilik basit mantık: Buffer dolunca Whisper'a gönder.
        // İleride buraya "Sessizlik Algılandığında Gönder" (VAD) eklenecek.
        if audio_buffer.len() >= flush_threshold {
            println!("[THINK] Ses işleniyor ({} samples)...", audio_buffer.len());
            
            // Transkripsiyon (Bloklayan işlem - Main thread durur)
            // DOOM Notu: İleride bu işlem async task olacak.
            if let Some(text) = brain.transcribe(&audio_buffer) {
                println!("\n>> DUYULAN: \"{}\"\n", text.trim());
            }

            // Buffer'ı temizle
            audio_buffer.clear();
        }

        // CPU Koruma
        thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}