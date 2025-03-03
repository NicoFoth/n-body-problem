pub(crate) struct Body {
    pub(crate) id: i32,
    pub(crate) position: Vector,
    pub(crate) velocity: Vector,
    pub(crate) mass: f64,
}

pub(crate) struct Vector {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Clone for Vector {
    fn clone(&self) -> Self {
        Vector {
            x: self.x,
            y: self.y,
        }
    }
}

impl Clone for Body {
    fn clone(&self) -> Self {
        Body {
            id: self.id,
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            mass: self.mass,
        }
    }
}


// Gravitational constant (using a value appropriate for our simulation scale)
const G: f64 = 1.0; 
// Smaller time step for better numerical stability
const TIME: f64 = 0.01;

pub(crate) fn update(bodies: &Vec<Body>) -> Vec<Body> {
    let mut new_bodies = Vec::new();
    
    // First calculate all accelerations
    for i in 0..bodies.len() {
        let mut body = bodies[i].clone();
        let mut total_force = Vector { x: 0.0, y: 0.0 };
        
        // Sum forces from all other bodies
        for j in 0..bodies.len() {
            if i != j {
                let force = calculate_force(&bodies[i], &bodies[j]);
                total_force = vector_add(&total_force, &force);
            }
        }
        
        // Update velocity using total force and TIME
        let acceleration = vector_div(&total_force, body.mass);
        body.velocity = vector_add(&body.velocity, &vector_mult(&acceleration, TIME));
        
        // Update position using new velocity
        body.position = vector_add(&body.position, &vector_mult(&body.velocity, TIME));
        
        new_bodies.push(body);
    }
    
    new_bodies
}

fn calculate_force(body1: &Body, body2: &Body) -> Vector {
    // Vector from body1 to body2
    let r_12: Vector = calculate_distance(&body1.position, &body2.position);
    let distance = calculate_length(&r_12);
    
    // Add small value to prevent division by zero
    let safe_distance = distance.max(1e-10);
    
    // For normalized direction vector, divide each component by distance
    let direction = Vector {
        x: r_12.x / safe_distance,
        y: r_12.y / safe_distance,
    };
    
    // Newton's law of gravity: F = G * m1 * m2 / r^2
    // For body1, the force is toward body2
    let force_magnitude = G * body1.mass * body2.mass / (safe_distance * safe_distance);
    
    // Apply magnitude to direction vector
    vector_mult(&direction, force_magnitude)
}

fn calculate_distance(vec1: &Vector, vec2: &Vector) -> Vector {
    Vector {
        x: vec2.x - vec1.x,
        y: vec2.y - vec1.y,
    }
}

fn calculate_length(vector: &Vector) -> f64 {
    (vector.x.powf(2.0) + vector.y.powf(2.0)).sqrt()
}

fn vector_mult(vector: &Vector, scalar: f64) -> Vector {
    Vector {
        x: vector.x * scalar,
        y: vector.y * scalar,
    }
}

fn vector_div(vector: &Vector, scalar: f64) -> Vector {
    Vector {
        x: vector.x / scalar,
        y: vector.y / scalar,
    }
}

fn vector_add(vector1: &Vector, vector2: &Vector) -> Vector {
    Vector {
        x: vector1.x + vector2.x,
        y: vector1.y + vector2.y,
    }
}