use nannou::prelude::*;
use crate::simulation::{Body, Vector, update};

pub(crate) struct Model {
    bodies: Vec<Body>,
    trails: Vec<Vec<Point2>>,
    scale: f32,
    speed: usize,
}

pub(crate) fn run() {
    nannou::app(model)
        .update(update_model)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .title("N-Body Simulation")
        .view(view)
        .build()
        .unwrap();

    // Setup a sun-like central body and an orbiting planet-like body
    let sun = Body {
        id: 0,
        position: Vector { x: 0.0, y: 0.0 },
        velocity: Vector { x: 0.0, y: 0.0 },  // Stationary sun
        mass: 1000.0,  // Much heavier
    };

    // Calculate proper orbital velocity for circular orbit
    // v = sqrt(G*M/r) where G=1, M=1000, r=5
    let orbital_velocity = f64::sqrt(1.0 * 1000.0 / 5.0);
    
    let planet = Body {
        id: 1,
        position: Vector { x: 5.0, y: 0.0 },  // Circular orbit at distance 5
        velocity: Vector { x: 0.0, y: orbital_velocity },  // Proper orbital velocity
        mass: 1.0,  // Much lighter
    };
    
    // Add a second planet
    let planet2 = Body {
        id: 2,
        position: Vector { x: 0.0, y: 8.0 },  // Different orbit
        velocity: Vector { x: -f64::sqrt(1.0 * 1000.0 / 8.0), y: 0.0 },
        mass: 0.8,
    };
    
    // Add a moon to the first planet
    let moon = Body {
        id: 3,
        position: Vector { x: 5.0, y: 0.5 },  // Orbiting first planet
        velocity: Vector { x: 0.0, y: orbital_velocity + 2.0 },
        mass: 0.1,
    };
    
    let bodies = vec![sun, planet, planet2, moon];
    let trails = vec![Vec::new(); bodies.len()];
    
    Model { 
        bodies, 
        trails, 
        scale: 40.0,  // Scaling factor for visualization
        speed: 1,     // Physics steps per frame
    }
}

fn update_model(app: &App, model: &mut Model, _update: Update) {
    // Run physics multiple times per frame based on speed setting
    for _ in 0..model.speed {
        // Update positions and velocities
        model.bodies = update(&model.bodies);
    }

    // Update trails
    for (i, body) in model.bodies.iter().enumerate() {
        let point = pt2(body.position.x as f32, body.position.y as f32);
        
        // Limit trail length to prevent memory issues
        if model.trails[i].len() > 500 {
            model.trails[i].remove(0);
        }
        
        model.trails[i].push(point);
    }

    // Handle keyboard input for adjusting simulation
    if app.keys.down.contains(&Key::Up) {
        model.speed = (model.speed + 1).min(10);
    }
    if app.keys.down.contains(&Key::Down) {
        model.speed = model.speed.saturating_sub(1).max(1);
    }
    if app.keys.down.contains(&Key::Right) {
        model.scale *= 1.05;
    }
    if app.keys.down.contains(&Key::Left) {
        model.scale *= 0.95;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    // Clear with dark background
    draw.background().color(BLACK);
    
    // Draw at the center of the window
    let win = app.window_rect();
    let center = win.xy();
    
    // Draw trails
    for (i, trail) in model.trails.iter().enumerate() {
        if trail.len() < 2 {
            continue;
        }
        
        let color = match i {
            0 => YELLOW,  // Sun
            1 => BLUE,    // Planet 1
            2 => RED,     // Planet 2
            3 => GREEN,   // Moon
            _ => PURPLE,  // Any other bodies
        };
        
        // Draw fading trail
        for j in 1..trail.len() {
            let alpha = ((j as f32 / trail.len() as f32) * 0.8 * 255.0) as u8;
            let trail_color = rgba(color.red, color.green, color.blue, alpha);
            
            draw.line()
                .start(center + trail[j-1] * model.scale)
                .end(center + trail[j] * model.scale)
                .color(trail_color)
                .stroke_weight(1.0);
        }
    }
    
    // Draw bodies
    for (i, body) in model.bodies.iter().enumerate() {
        let position = pt2(body.position.x as f32, body.position.y as f32);
        let size = (body.mass as f32).sqrt() * 2.0;
        
        let color = match i {
            0 => YELLOW,  // Sun
            1 => BLUE,    // Planet 1
            2 => RED,     // Planet 2
            3 => GREEN,   // Moon
            _ => PURPLE,  // Any other bodies
        };
        
        draw.ellipse()
            .xy(center + position * model.scale)
            .radius(size)
            .color(color);
    }
    
    // Draw info text
    draw.text(&format!("Speed: {}x", model.speed))
        .xy(win.top_left() + vec2(80.0, -20.0))
        .color(WHITE);
    
    draw.text("Controls: Arrow Keys (Up/Down=Speed, Left/Right=Zoom)")
        .xy(win.bottom_left() + vec2(200.0, 20.0))
        .color(WHITE);
    
    // Finish drawing
    draw.to_frame(app, &frame).unwrap();
}