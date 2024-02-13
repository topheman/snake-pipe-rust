use std::time::{Duration, Instant};

use crate::common::format_version_to_display;
use crate::stream::parse_gamestate;

const FRAME_ACCURACY: Duration = Duration::from_millis(20);

pub fn run(frame_duration: u32) {
    let frame_duration_millis = Duration::from_millis(frame_duration as u64);
    match parse_gamestate() {
        Ok(mut stream) => {
            let mut options_passthrough = stream.options.clone();
            options_passthrough.frame_duration = frame_duration;
            options_passthrough
                .features_with_version
                .insert("throttle".to_string(), format_version_to_display());
            println!("{}\r", serde_json::to_string(&options_passthrough).unwrap());
            let mut last_loop_duration: Duration = Duration::new(0, 0);
            loop {
                let start = Instant::now();
                while start.elapsed() < FRAME_ACCURACY {
                    std::hint::spin_loop();
                }
                if last_loop_duration > frame_duration_millis {
                    if let Some(parsed_line) = stream.lines.next() {
                        println!("{}\r", serde_json::to_string(&parsed_line).unwrap());
                        // adjust framerate
                        let remainder = last_loop_duration - frame_duration_millis;
                        last_loop_duration = remainder;
                    } else {
                        std::process::exit(0);
                    }
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
