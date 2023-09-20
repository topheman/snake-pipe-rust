use crate::stream::{parse_gamestate, GameState};

pub fn run() {
    let stream = parse_gamestate();
    println!("options: {:?}\r", stream.options);
    for parsed_line in stream.lines {
        println!("state: {:?}\r", parsed_line);
    }
}

fn render_frame(state: GameState) {
    println!("{:?}\r", state);
}
