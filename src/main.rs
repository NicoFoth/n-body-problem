mod simulation;

fn main() {
    // Setup a sun-like central body and an orbiting planet-like body
    let sun = simulation::Body {
        id: 0,
        position: simulation::Vector { x: 0.0, y: 0.0 },
        velocity: simulation::Vector { x: 0.0, y: 0.0 },  // Stationary sun
        mass: 1000.0,  // Much heavier
    };

    // Calculate proper orbital velocity for circular orbit
    // v = sqrt(G*M/r) where G=1, M=1000, r=5
    let orbital_velocity = f64::sqrt(1.0 * 1000.0 / 5.0);
    println!("Using orbital velocity: {}", orbital_velocity);
    
    let planet = simulation::Body {
        id: 1,
        position: simulation::Vector { x: 5.0, y: 0.0 },  // Circular orbit at distance 5
        velocity: simulation::Vector { x: 0.0, y: orbital_velocity },  // Proper orbital velocity
        mass: 1.0,  // Much lighter
    };
    
    let mut bodies = vec![sun, planet];
    
    // Run simulation for more steps
    println!("Starting simulation...");
    for step in 0..100 {
        if step % 10 == 0 {
            println!("\nStep {}:", step);
            for body in &bodies {
                println!("Body {}: pos=({:.4}, {:.4}), vel=({:.4}, {:.4})", 
                         body.id, body.position.x, body.position.y, 
                         body.velocity.x, body.velocity.y);
            }
        }
        bodies = simulation::update(&bodies);
    }
    
    println!("\nFinal state:");
    for body in bodies {
        println!("Body {}: pos=({:.4}, {:.4}), vel=({:.4}, {:.4})", 
                 body.id, body.position.x, body.position.y, 
                 body.velocity.x, body.velocity.y);
    }
}