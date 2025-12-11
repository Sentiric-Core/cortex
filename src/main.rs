/*
 * SENTIRIC CORTEX ENGINE
 * ----------------------
 * CORE LOOP v1.1 - The Senses
 */

mod audio; // Audio modülünü içeri aktar

use std::time::{Instant, Duration};
use std::thread;
use crate::audio::Ear;

struct EngineState {
    is_running: bool,
    ticks: u64,
}

fn main() {
    println!("-------------------------------------------");
    println!("SENTIRIC CORTEX ENGINE: BOOT SEQUENCE");
    println!("-------------------------------------------");

    let mut state = EngineState {
        is_running: true,
        ticks: 0,
    };

    // 1. DONANIM BAŞLATMA (Ear Modülü)
    // Hata olursa motoru durdurma, sadece raporla (Fault Tolerance)
    let ear = match Ear::new() {
        Ok(e) => Some(e),
        Err(e) => {
            eprintln!("[CRITICAL] İşitme modülü başlatılamadı: {}", e);
            None
        }
    };

    let engine_start = Instant::now();
    println!("[SYSTEM] Motor döngüsü (Heartbeat) başladı...");

    // ANA MOTOR DÖNGÜSÜ
    while state.is_running {
        state.ticks += 1;
        
        // --- 1. SENSE (İşitme) ---
        if let Some(ref active_ear) = ear {
            // Mikrofondan veri geldi mi?
            // Döngü çok hızlı olduğu için her tick'te veri gelmeyebilir.
            if let Some(packet) = active_ear.listen() {
                // Sadece görselleştirme amaçlı: Gelen sesin şiddetini (RMS) ölç
                let rms: f32 = (packet.samples.iter().map(|x| x * x).sum::<f32>() / packet.samples.len() as f32).sqrt();
                
                // Eğer ses belli bir eşiğin üzerindeyse log bas (Silence Detection)
                if rms > 0.01 { 
                    println!("[SENSE] Ses Algılandı | Şiddet: {:.4} | Buffer: {} samples", rms, packet.samples.len());
                }
            }
        }

        // --- 2. THINK ---
        // (Henüz boş)

        // --- 3. ACT ---
        // (Henüz boş)

        // İstatistik Raporu (Her 5 saniyede bir)
        if state.ticks % 5000 == 0 { // 1ms sleep ile yaklaşık 5 saniye
            let uptime = engine_start.elapsed().as_secs();
            println!("[HEARTBEAT] Uptime: {}s | Ticks: {}", uptime, state.ticks);
        }

        // CPU Koruma (1ms uyku = ~1000 FPS)
        thread::sleep(Duration::from_millis(1));
    }

    println!("SENTIRIC CORTEX ENGINE: SHUTDOWN");
}