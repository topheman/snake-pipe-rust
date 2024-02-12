use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{stdin, BufRead, Lines};

// options

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SizeOption {
    pub width: u32,
    pub height: u32,
}

/// Holds the options that were passed to the cli with a flag
/// that are relevent for rendering the game.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InitOptions {
    pub frame_duration: u32,
    #[serde(skip)]
    pub snake_length: u32,
    pub size: SizeOption,
}

// gamestate

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Snake {
    pub direction: Direction,
    pub head: Position,
    pub tail: Vec<Position>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GameState {
    Paused,
    Over,
    Running,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::Over => "Game Over",
            Self::Paused => "Paused",
            Self::Running => "Running",
        };
        write!(f, "{}", str)
    }
}

/// Holds the state of the game at any time
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    pub snake: Snake,
    pub fruit: Position,
    pub score: u32,
    pub state: GameState,
}

/// Accepts the iterator from `stdin().lines()`
/// - parses the first line into `options` as [InitOptions]
/// - returns an iterator of [Game] inside `lines` (already parsed)
///
/// Used by [parse_gamestate] under the hood.
pub struct Stream {
    pub options: InitOptions,
    pub lines: Box<dyn Iterator<Item = Game>>, // std::io::Lines<T>, //Lines<T>,
}

impl Stream {
    fn new<T: BufRead + 'static>(
        mut lines: Lines<T>,
    ) -> Result<Stream, Box<dyn std::error::Error>> {
        let first_line = lines.next().unwrap()?;
        let options: InitOptions = serde_json::from_str(&first_line)?;
        // flat_map keeps Some and extracts their values while removing Err - we ignore parse errors on lines / we dont panic on it
        let parsed_lines = lines.flat_map(|result_line| match result_line {
            Ok(line) => match serde_json::from_str::<Game>(&line) {
                Ok(parsed_line) => Some(parsed_line),
                Err(_) => None,
            },
            Err(_) => None,
        });
        Ok(Self {
            options,
            lines: Box::new(parsed_lines),
        })
    }
}

/// Parses a gamestate streamed into stdin
/// Example:
/// ```
/// use snakepipe::stream::{parse_gamestate, Game};
///
/// match parse_gamestate() {
///     Ok(stream) => {
///         println!(
///             "Frame duration {}, Snake length {}, Level {}x{}",
///             stream.options.frame_duration,
///             stream.options.snake_length,
///             stream.options.size.width,
///             stream.options.size.height
///         );
///         for parsed_line in stream.lines {
///             do_something(parsed_line);
///         }
///     }
///     Err(e) => {
///         println!("Error occurred while parsing stdin: \"{}\"", e);
///     }
/// }
///
/// fn do_something(parsed_line: Game) {
///     println!("Snake head position {:?}", parsed_line.snake.head)
/// }
/// ```
pub fn parse_gamestate() -> Result<Stream, Box<dyn std::error::Error>> {
    let lines = stdin().lines();
    Stream::new(lines)
}
