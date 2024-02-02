use std::io::Write;

use crate::stream::{parse_gamestate, Direction as StreamDirection, Game, GameState};
use array2d::Array2D;
use crossterm::{cursor, queue, style, terminal};

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<StreamDirection> for Direction {
    fn from(value: StreamDirection) -> Self {
        match value {
            StreamDirection::Up => Direction::Up,
            StreamDirection::Down => Direction::Down,
            StreamDirection::Left => Direction::Left,
            StreamDirection::Right => Direction::Right,
        }
    }
}

#[derive(Clone, Debug)]
enum Point {
    Head(Direction),
    Tail,
    Fruit,
    Nothing,
}

#[derive(Debug)]
struct RenderGrid {
    data: Array2D<Point>,
}

impl RenderGrid {
    fn new(width: u32, height: u32) -> Self {
        RenderGrid {
            data: Array2D::filled_with(Point::Nothing, height as usize, width as usize),
        }
    }
    fn set(&mut self, x: usize, y: usize, point: Point) {
        let _ = self.data.set(y, x, point);
    }
}

pub fn run() {
    match parse_gamestate() {
        Ok(stream) => {
            // println!("{:?}\n", &stream.options);
            let mut stdout = std::io::stdout();
            queue!(
                stdout,
                terminal::Clear(terminal::ClearType::All),
                cursor::Hide,
                cursor::MoveTo(0, 0),
                cursor::SavePosition,
            )
            .unwrap();
            for parsed_line in stream.lines {
                let mut grid =
                    RenderGrid::new(stream.options.size.width, stream.options.size.height);
                prepare_grid(&mut grid, parsed_line.clone());
                render_frame(
                    &grid,
                    stream.options.size.width,
                    parsed_line.score,
                    parsed_line.state,
                    &mut stdout,
                );
                stdout.flush().unwrap();
            }
            // once there is no more stream (maybe ctrl-c), show the cursor back
            queue!(stdout, cursor::Show, cursor::RestorePosition,).unwrap();
        }
        Err(e) => {
            println!("Error occurred while parsing stdin: \"{}\"", e);
        }
    }
}

fn prepare_grid(grid: &mut RenderGrid, game_state: Game) {
    let direction: Direction = game_state.snake.direction.into();
    grid.set(
        game_state.snake.head.x as usize,
        game_state.snake.head.y as usize,
        Point::Head(direction.clone()),
    );
    game_state.snake.tail.into_iter().for_each(|f| {
        grid.set(f.x as usize, f.y as usize, Point::Tail);
    });
    grid.set(
        game_state.fruit.x as usize,
        game_state.fruit.y as usize,
        Point::Fruit,
    );
}

/**
 * https://en.wikipedia.org/wiki/Box-drawing_character
 */
fn render_line_wrapper(width: u32, top: bool) -> String {
    let line = (0..width)
        .into_iter()
        .fold("".to_string(), |acc, _| format!("{}{}", acc, "-"));
    match top {
        true => format!("{}{}{}", "\u{250C}", line, "\u{2510}"),
        false => format!("{}{}{}", "\u{2514}", line, "\u{2518}"),
    }
}

fn render_frame(
    grid: &RenderGrid,
    width: u32,
    score: u32,
    state: GameState,
    stdout: &mut std::io::Stdout,
) {
    queue!(
        stdout,
        cursor::RestorePosition,
        style::Print(render_line_wrapper(width, true)),
        cursor::MoveToNextLine(1)
    )
    .unwrap();
    grid.data.rows_iter().for_each(|row| {
        let row_reduced: String = row.into_iter().fold("".to_string(), |row_acc, cell| {
            let cell_content = match cell {
                Point::Fruit => "F",
                Point::Head(_) => "H",
                Point::Nothing => "·",
                Point::Tail => "T",
            };
            format!("{}{}", row_acc, cell_content)
        });
        queue!(
            stdout,
            style::Print(format!("│{}│", row_reduced)),
            cursor::MoveToNextLine(1)
        )
        .unwrap();
    });
    queue!(
        stdout,
        style::Print(render_line_wrapper(width, false)),
        cursor::MoveToNextLine(1),
        style::Print(format!("Score: {} - {}     ", score, state)),
        cursor::MoveToNextLine(1),
        style::Print(format!("[P] Pause")),
        cursor::MoveToNextLine(1),
        style::Print(format!("[R] Restart")),
        cursor::MoveToNextLine(1),
        style::Print(format!("[Ctrl+C] Quit")),
    )
    .unwrap();
}
