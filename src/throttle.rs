use std::time::{Duration, Instant};

use crate::common::format_version_to_display;
use crate::input::{parse_gamestate, Game};

const FRAME_ACCURACY: Duration = Duration::from_millis(20);

pub fn run(frame_duration: u32, loop_infinite: bool) {
    let frame_duration_millis = Duration::from_millis(frame_duration as u64);
    let mut recording_buffer: Vec<Game> = Vec::new();
    match parse_gamestate() {
        Ok(mut input) => {
            let mut options_passthrough = input.options.clone();
            options_passthrough.frame_duration = frame_duration;
            options_passthrough
                .features_with_version
                .insert("throttle".to_string(), format_version_to_display());
            options_passthrough
                .metadatas
                .insert("throttled".to_string(), "on".to_string());
            println!("{}\r", serde_json::to_string(&options_passthrough).unwrap());
            let mut last_loop_duration: Duration = Duration::new(0, 0);
            let mut replaying_index = 0;
            loop {
                let start = Instant::now();
                while start.elapsed() < FRAME_ACCURACY {
                    std::hint::spin_loop();
                }
                if last_loop_duration > frame_duration_millis {
                    if let Some(parsed_line) = input.lines.next() {
                        recording_buffer.push(parsed_line.clone());
                        println!("{}\r", serde_json::to_string(&parsed_line).unwrap());
                    } else {
                        if !loop_infinite {
                            std::process::exit(0);
                        }
                        replaying_index = if replaying_index < recording_buffer.len() {
                            replaying_index
                        } else {
                            0
                        };
                        println!(
                            "{}\r",
                            serde_json::to_string(recording_buffer.get(replaying_index).unwrap())
                                .unwrap()
                        );
                        replaying_index = replaying_index + 1;
                    }
                    // adjust framerate
                    let remainder = last_loop_duration - frame_duration_millis;
                    last_loop_duration = remainder;
                }
                last_loop_duration += start.elapsed();
            }
        }
        Err(e) => {
            eprintln!("Error occurred while parsing stdin: \"{}\"", e);
            std::process::exit(1);
        }
    }
}
