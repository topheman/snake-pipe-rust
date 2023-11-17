use crate::stream::{parse_gamestate, Direction as StreamDirection, GameState};
use array2d::Array2D;

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
            data: Array2D::filled_with(Point::Nothing, width as usize, height as usize),
        }
    }
    fn set(&mut self, x: usize, y: usize, point: Point) {
        let _ = self.data.set(x, y, point);
    }
}

pub fn run() {
    match parse_gamestate() {
        Ok(stream) => {
            println!("{:?}\n", &stream.options);
            for parsed_line in stream.lines {
                let mut grid =
                    RenderGrid::new(stream.options.size.width, stream.options.size.height);
                // println!("{:?}", &parsed_line);
                prepare_grid(&mut grid, parsed_line);
                // println!("{:?}", &grid);
                let frame = render_frame(&grid);
                println!("{}", frame);
            }
        }
        Err(e) => {
            println!("Error occurred while parsing stdin: \"{}\"", e);
        }
    }
}

fn prepare_grid(grid: &mut RenderGrid, game_state: GameState) {
    let direction: Direction = game_state.snake.direction.into();
    grid.set(
        game_state.snake.head.x as usize,
        game_state.snake.head.y as usize,
        Point::Head(direction),
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

fn render_frame(grid: &RenderGrid) -> String {
    let output = grid.data.rows_iter().fold("".to_string(), |acc, row| {
        let row_reduced: String = row.into_iter().fold("".to_string(), |row_acc, cell| {
            let cell_content = match cell {
                Point::Fruit => "F",
                Point::Head(_) => "H",
                Point::Nothing => " ",
                Point::Tail => "T",
            };
            format!("{}{}", row_acc, cell_content)
        });
        return format!("{}{}\n", acc, row_reduced);
    });
    return output;
}
