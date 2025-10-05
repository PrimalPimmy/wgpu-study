
use bevy_math::Vec2;

pub struct Blackhole {
    pub position: Vec2, // f32-based
    pub mass: f64,
    pub radius: f64,
}

const G: f64 = 6.67430e-11;
const C: f64 = 299_792_458.0;

impl Blackhole {
    pub fn new(pos: Vec2, mass: f64) -> Self {
        let radius = (2.0 * G * mass) / (C * C);
        Self { position: pos, mass, radius }
    }

    // Build a filled circle (triangle list) centered at self.position in clip space
    // segments >= 3; color as [f32; 3]
    pub fn circle_filled_vertices(&self, segments: usize, color: [f32; 3])
        -> (Vec<([f32; 3], [f32; 3])>, Vec<u16>)
    {
        assert!(segments >= 3);
        let r = self.radius as f32;
        let cx = self.position.x;
        let cy = self.position.y;

        // vertices: (position, color)
        let mut verts: Vec<([f32; 3], [f32; 3])> = Vec::with_capacity(segments + 1);
        let mut indices: Vec<u16> = Vec::with_capacity(segments * 3);

        // center
        verts.push(([cx, cy, 0.0], color));

        // perimeter
        for i in 0..segments {
            let angle = (i as f32) * std::f32::consts::TAU / (segments as f32);
            let x = cx + r * angle.cos();
            let y = cy + r * angle.sin();
            verts.push(([x, y, 0.0], color));
        }

        // triangles (0, i, i+1) and close
        for i in 1..segments {
            indices.extend_from_slice(&[0, i as u16, (i as u16) + 1]);
        }
        indices.extend_from_slice(&[0, segments as u16, 1]);

        (verts, indices)
    }
}
