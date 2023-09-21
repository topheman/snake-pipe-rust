use crate::stream::{parse_gamestate, GameState};

pub fn run() {
    match parse_gamestate() {
        Ok(stream) => {
            println!("options: {:?}\r", stream.options);
            for parsed_line in stream.lines {
                println!("state: {:?}\r", parsed_line);
            }
        }
        Err(e) => {
            println!("Error occurred while parsing stdin: \"{}\"", e);
        }
    }
}

fn render_frame(state: GameState) {
    println!("{:?}\r", state);
}
