use glam::Vec2;
use crate::circle::Vertex;

// The C constant from lib.rs, but as f32 for consistency with Vertex data
const C: f32 = 299792458.0;

pub struct Ray {
    pub x: f32,
    pub y: f32,
    pub dir: Vec2,
}

impl Ray {
    pub fn new(x: f32, y: f32, dir: Vec2) -> Self {
        Self {
            x,
            y,
            dir: dir.normalize(),
        }
    }

    pub fn step(&mut self) {
        // Using a fraction of C for visible movement
        let speed = C * 0.00000001;
        self.x += self.dir.x * speed;
        self.y += self.dir.y * speed;
    }

    pub fn to_quad_vertices(&self, size: f32) -> [Vertex; 4] {
        let half_size = size / 2.0;
        [
            Vertex { // Top-left
                position: [self.x - half_size, self.y + half_size, 0.0],
                color: [1.0, 1.0, 0.0],
            },
            Vertex { // Top-right
                position: [self.x + half_size, self.y + half_size, 0.0],
                color: [1.0, 1.0, 0.0],
            },
            Vertex { // Bottom-left
                position: [self.x - half_size, self.y - half_size, 0.0],
                color: [1.0, 1.0, 0.0],
            },
            Vertex { // Bottom-right
                position: [self.x + half_size, self.y - half_size, 0.0],
                color: [1.0, 1.0, 0.0],
            },
        ]
    }
}