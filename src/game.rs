use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    pub snake: Vec<Position>,
    pub food: Position,
    pub direction: Direction,
    pub next_direction: Direction,
    pub grid_size: i32,
    pub score: u32,
    pub game_over: bool,
}

impl Game {
    pub fn new(grid_size: i32) -> Self {
        let mut game = Self {
            snake: vec![Position { x: 10, y: 10 }, Position { x: 10, y: 11 }, Position { x: 10, y: 12 }],
            food: Position { x: 5, y: 5 },
            direction: Direction::Up,
            next_direction: Direction::Up,
            grid_size,
            score: 0,
            game_over: false,
        };
        game.spawn_food();
        game
    }

    pub fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let new_food = Position {
                x: rng.gen_range(0..self.grid_size),
                y: rng.gen_range(0..self.grid_size),
            };
            if !self.snake.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }

    pub fn step(&mut self) {
        if self.game_over {
            return;
        }

        self.direction = self.next_direction;
        let head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => Position { x: head.x, y: head.y - 1 },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x - 1, y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        };

        // Wall collision
        if new_head.x < 0 || new_head.x >= self.grid_size || new_head.y < 0 || new_head.y >= self.grid_size {
            self.game_over = true;
            return;
        }

        // Self collision
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, new_head);

        // Food collision
        if new_head == self.food {
            self.score += 10;
            self.spawn_food();
        } else {
            self.snake.pop();
        }
    }

    pub fn change_direction(&mut self, dir: Direction) {
        match (self.direction, dir) {
            (Direction::Up, Direction::Down) => {}
            (Direction::Down, Direction::Up) => {}
            (Direction::Left, Direction::Right) => {}
            (Direction::Right, Direction::Left) => {}
            _ => self.next_direction = dir,
        }
    }
}
