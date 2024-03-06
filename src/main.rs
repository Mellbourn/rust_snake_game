extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;

struct Game {
    snake: Snake,
    food: Option<[i32; 2]>,
}

impl Game {
    fn render(&self, con: &Context, g: &mut G2d) {
        if let Some(food) = self.food {
            rectangle(
                [0.8, 0.0, 0.0, 1.0], // red
                [food[0] as f64 * 20.0, food[1] as f64 * 20.0, 20.0, 20.0],
                con.transform,
                g,
            );
        }

        self.snake.render(con, g);
    }

    fn update(&mut self) {
        self.snake.move_forward();

        if let Some(food) = self.food {
            if self.snake.head() == food {
                self.snake.grow();
                self.food = None;
            }
        }

        if self.food.is_none() {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            self.food = Some([x, y]);
        }
    }
}

struct Snake {
    body: Vec<[i32; 2]>,
    dir: Direction,
}

impl Snake {
    fn render(&self, con: &Context, g: &mut G2d) {
        for segment in &self.body {
            rectangle(
                [0.0, 0.8, 0.0, 1.0], // green
                [segment[0] as f64 * 20.0, segment[1] as f64 * 20.0, 20.0, 20.0],
                con.transform,
                g,
            );
        }
    }

    fn move_forward(&mut self) {
        let mut new_head = [self.head()[0], self.head()[1]];

        match self.dir {
            Direction::Right => new_head[0] += 1,
            Direction::Left => new_head[0] -= 1,
            Direction::Up => new_head[1] -= 1,
            Direction::Down => new_head[1] += 1,
        }

        self.body.insert(0, new_head);
        self.body.pop();
    }

    fn grow(&mut self) {
        let new_segment = [self.body.last().unwrap()[0], self.body.last().unwrap()[1]];
        self.body.push(new_segment);
    }

    fn head(&self) -> [i32; 2] {
        self.body[0]
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [400, 400])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        snake: Snake {
            body: vec![[WIDTH / 2, HEIGHT / 2]],
            dir: Direction::Right,
        },
        food: None,
    };

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.snake.dir = match key {
                Key::Right => Direction::Right,
                Key::Left => Direction::Left,
                Key::Up => Direction::Up,
                Key::Down => Direction::Down,
                _ => game.snake.dir,
            };
        }

        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            game.render(&c, g);
        });

        event.update(|_| {
            game.update();
        });
    }
}
