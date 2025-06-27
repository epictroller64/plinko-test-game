use macroquad::prelude::*;

struct BlinkoBall {
    x: f32,
    y: f32,
    r: f32,
    color: Color,
}

impl BlinkoBall {
    pub fn new(x: f32, y: f32, r: f32, color: Color) -> Self {
        Self { x, y, r, color }
    }

    pub fn can_move(&self, direction: Direction) -> bool {
        const MOVE_INTERVAL: f32 = 1.0;
        match direction {
            Direction::Down => self.y + MOVE_INTERVAL <= screen_height() - self.r,
            Direction::Up => self.y - MOVE_INTERVAL >= 0.0 + self.r,
            Direction::Right => self.x + MOVE_INTERVAL <= screen_width() - self.r,
            Direction::Left => self.x - MOVE_INTERVAL >= 0.0 + self.r,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.r, self.color);
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn move_circle(&mut self, direction: Direction) {
        const STEP: f32 = 1.0;
        match direction {
            Direction::Left => self.set_x(self.x - STEP),
            Direction::Right => self.set_x(self.x + STEP),
            Direction::Down => self.set_y(self.y + STEP),
            Direction::Up => self.set_y(self.y - STEP),
        }
    }
}

struct Bubble {
    x: f32,
    y: f32,
    r: f32,
    color: Color,
    index: i32,
}

impl Bubble {
    pub fn new(x: f32, y: f32, r: f32, color: Color, index: i32) -> Self {
        Self {
            x,
            y,
            r,
            color,
            index,
        }
    }

    pub fn does_collide(&self, x: f32, y: f32, r: f32) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        let distance_squared = dx * dx + dy * dy;
        let radius_sum = self.r + r;
        distance_squared <= radius_sum * radius_sum
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.r, self.color);

        let text = format!("{}", self.index);
        let font_size = self.r; // or any value you like
        let text_dimensions = measure_text(&text, None, font_size as u16, 1.0);
        let text_x = self.x - text_dimensions.width / 2.0;
        let text_y = self.y + text_dimensions.height / 2.0;
        draw_text(&text, text_x, text_y, font_size, BLACK);
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut blinko = BlinkoBall::new(screen_width() / 2.0, 9.0, 9.0, GREEN);
    let bubbles = draw_bubbles();
    let mut move_in_progress = false;
    let mut curr_dir = Direction::Right;
    let mut move_counter = 0;
    loop {
        let mut does_collide = false;
        for bubble in &bubbles {
            if !does_collide {
                if bubble.does_collide(blinko.x, blinko.y, blinko.r) {
                    // Collides, now we should send it left or right, and until it gets away from it
                    does_collide = true;
                }
            }
            bubble.draw();
        }
        if does_collide {
            if move_in_progress {
                blinko.move_circle(curr_dir.clone());
                move_counter += 1;
            } else {
                if rand::gen_range(0, 2) == 0 {
                    blinko.move_circle(Direction::Right);
                    move_in_progress = true;
                    curr_dir = Direction::Right;
                } else {
                    blinko.move_circle(Direction::Left);
                    move_in_progress = true;
                    curr_dir = Direction::Left;
                }
            }
        } else {
            const STOPPER: i32 = 25;
            if move_counter >= STOPPER {
                move_counter = 0;
                move_in_progress = false;
            }
            blinko.move_circle(Direction::Down);
        }
        blinko.draw();
        if blinko.y > screen_height() {
            // create new one
            blinko = BlinkoBall::new(screen_width() / 2.0, 10.0, 10.0, GREEN);
        }
        next_frame().await
    }
}

fn blinko_drop(blinko: &BlinkoBall) {
    // Drop the blinko ball further and check for colissions
}

fn draw_bubbles() -> Vec<Bubble> {
    let w = screen_width();
    let h = screen_height();
    let padding = 20.0;
    let r = 5.0;
    let total_bubble_diameter = r * 2.0 + padding;
    //let mut max_bubbles_per_row = ((w + padding) / total_bubble_diameter).floor() as i32;
    let mut max_bubbles_per_row = 15;
    let mut bubbles: Vec<Bubble> = Vec::new();
    let mut y = h - r - padding;
    let mut index = 0;
    while max_bubbles_per_row >= 0 {
        let row_width = max_bubbles_per_row as f32 * total_bubble_diameter - padding;
        // Center the row
        let start_x = (w - row_width) / 2.0 + r;

        for i in 0..max_bubbles_per_row {
            let x = start_x + i as f32 * total_bubble_diameter;
            let bubble = Bubble {
                x,
                y,
                r,
                color: YELLOW,
                index,
            };
            bubbles.push(bubble);
            index += 1;
        }
        max_bubbles_per_row -= 1;
        y -= total_bubble_diameter;
    }
    bubbles
}

fn blinko_ball_movement() {
    let mut circle = BlinkoBall::new(50.0, 50.0, 15.0, GREEN);
    let mut x_direction = Direction::Left;
    let mut y_direction = Direction::Down;
    if circle.can_move(x_direction.clone()) {
        circle.move_circle(x_direction.clone());
    } else {
        x_direction = match x_direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            _ => x_direction, // Only reverse if Left/Right
        };
    }

    // Handle Y direction
    if circle.can_move(y_direction.clone()) {
        circle.move_circle(y_direction.clone());
    } else {
        y_direction = match y_direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            _ => y_direction, // Only reverse if Up/Down
        };
    }

    circle.draw();
}

#[derive(PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}
