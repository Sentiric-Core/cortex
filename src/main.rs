/*
 * SENTIRIC CORTEX ENGINE
 * ----------------------
 * CORE LOOP v2.0 - The Awakening
 */

mod audio;
mod cognition;
mod synthesis;
mod intelligence; // YENİ

use std::time::Duration;
use std::thread;
use crate::audio::Ear;
use crate::cognition::Brain;
use crate::synthesis::Mouth;
use crate::intelligence::Mind;

struct EngineState {
    is_running: bool,
    ticks: u64,
}

fn calculate_rms(samples: &[f32]) -> f32 {
    let sum_squares: f32 = samples.iter().map(|&x| x * x).sum();
    (sum_squares / samples.len() as f32).sqrt()
}

fn sanitize_output(text: &str) -> Option<String> {
    let raw = text.replace("[BLANK_AUDIO]", ""); 
    let raw = raw.trim();
    if raw.is_empty() { return None; }
    if raw.starts_with('(') || raw.starts_with('[') { return None; }
    if raw.len() < 2 { return None; }
    Some(raw.to_string())
}

fn main() -> anyhow::Result<()> {
    println!("-------------------------------------------");
    println!("SENTIRIC CORTEX ENGINE: LOADING CORES...");
    println!("-------------------------------------------");

    let mut state = EngineState { is_running: true, ticks: 0 };
    
    // 1. Modüller
    let ear = Ear::new()?;
    let brain = Brain::new()?;
    let mind = Mind::new()?; // YENİ: Llama Yükleniyor (Biraz sürebilir)
    let mouth = Mouth::new()?;

    let mut audio_buffer: Vec<f32> = Vec::new();
    let flush_threshold = 16000 * 3; 
    let silence_threshold = 0.020;    

    println!("-------------------------------------------");
    println!("SYSTEM ONLINE. NEURAL NETWORKS READY.");
    println!("-------------------------------------------");
    
    mouth.speak("Sentiric core online.");
    ear.clear_buffer();

    while state.is_running {
        state.ticks += 1;
        
        // SENSE
        if let Some(packet) = ear.listen() {
            audio_buffer.extend_from_slice(&packet.samples);
        }

        // PERCEIVE
        if audio_buffer.len() >= flush_threshold {
            let loudness = calculate_rms(&audio_buffer);

            if loudness > silence_threshold {
                print!("[INPUT] (RMS: {:.4}) İşleniyor... ", loudness);
                
                if let Some(input_text) = brain.transcribe(&audio_buffer) {
                    if let Some(clean_input) = sanitize_output(&input_text) {
                        println!("\n>> KULLANICI: \"{}\"", clean_input);
                        
                        // THINK (Echo yerine Intelligence)
                        if let Some(response) = mind.think(&clean_input) {
                            println!(">> SENTIRIC:  \"{}\"", response);
                            
                            // ACT
                            mouth.speak(&response);
                            
                            // REFLEXIVE DEAFNESS
                            ear.clear_buffer();
                            println!("[SYSTEM] Tampon temizlendi.");
                        }

                    } else {
                        println!("(Gürültü filtrelendi)");
                    }
                }
            }
            audio_buffer.clear();
        }

        thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}