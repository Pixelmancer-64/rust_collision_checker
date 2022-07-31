use nannou::{color::named, prelude::*};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
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

        
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            pos: Position::new(0.0,0.0),
            vel: Position::new(1.0,1.0),
            radius: 10.0,
        }
    }
}

struct Model {
    point: Point,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            point: Point::new(),
        }
    }
}

impl Nannou for Model {
    /// Show this model
    fn display(&self, draw: &Draw) {
        // draw.background().color(self.bg_color);
        self.point.display(draw);
    }
    /// Update this model
    fn update(&mut self) {
        self.point.update();
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