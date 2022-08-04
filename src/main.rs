use nannou::{color::named, prelude::*};

fn main() {
    nannou::app(model).update(update).size(1000,1000).simple_window(view).run();
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Things that can be drawn to the screen
trait Nannou {
    fn display(&self, draw: &Draw);
    fn update(&mut self);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    pos: Position,
    vel: Position,
    radius: f32,
}

impl Point {
    fn new() -> Self {
        Self::default()
    }
}

impl Nannou for Point {
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .w(self.radius)
            .h(self.radius)
            .x_y(self.pos.x, self.pos.y)
            .color(named::STEELBLUE);
    }

    fn update(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        if self.pos.x + self.radius > 500.0{
            self.vel.x = -self.vel.x
        } else if self.pos.x - self.radius< -500.0{
            self.vel.x = -self.vel.x
        }

        if self.pos.y + self.radius > 500.0{
            self.vel.y = -self.vel.y
        } else if self.pos.y - self.radius< -500.0{
            self.vel.y = -self.vel.y
        }
    }
}

fn random_negative(range: f32) -> f32{
    random_f32() * range * if random_f32() > 0.5 {1.0} else {-1.0}
}

impl Default for Point {
    fn default() -> Self {
        Self {
            pos: Position::new(random_f32() * 100.0, random_f32() * 100.0),
            vel: Position::new(random_negative(3.0), random_negative(3.0)),
            radius: 10.0,
        }
    }
}

struct Model {
    points: Vec<Point>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            points: {
                let mut ret = Vec::new();
                for _ in 0..300 {
                    ret.push(Point::new());
                }
                ret
            },
        }
    }
}

impl Nannou for Model {
    /// Show this model
    fn display(&self, draw: &Draw) {
        draw.background().color(BLACK);
        self.points.iter().for_each(|e| e.display(draw));
    }
    /// Update this model
    fn update(&mut self) {
        self.points.iter_mut().for_each(|e| e.update());
    }
}

/// Nannou app model
fn model(_app: &App) -> Model {
    Model::default()
}

/// Nannou app update
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

/// Nannou app view
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // Draw model
    model.display(&draw);
    // Render frame
    draw.to_frame(&app, &frame).unwrap();
}
