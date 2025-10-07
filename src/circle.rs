use glam::Vec3;

// Using the same constants for physical calculations
const G: f64 = 6.67430e-11;
const C: f64 = 299792458.0;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Blackhole {
    pub position: Vec3,
    pub mass: f64,
    pub radius: f64, // Schwarzschild radius
}

impl Blackhole {
    pub fn new(position: Vec3, mass: f64) -> Self {
        let radius = (2.0 * G * mass) / (C * C);
        Self {
            position,
            mass,
            radius,
        }
    }

    /// Generates the vertex and index data for a filled circle representing the black hole.
    pub fn draw_circle(&self, segments: usize) -> (Vec<Vertex>, Vec<u16>) {
        assert!(segments >= 3);
        let mut verts = Vec::with_capacity(segments + 1);
        let mut indices = Vec::with_capacity(segments * 3);

        // As in the C++ example, we'll use a red color
        let color = [1.0, 0.0, 0.0];

        // Center vertex
        verts.push(Vertex {
            position: self.position.to_array(),
            color,
        });

        // Outer vertices
        for i in 0..segments {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (segments as f64);
            let x_offset = self.radius * angle.cos();
            let y_offset = self.radius * angle.sin();
            verts.push(Vertex {
                position: [
                    (x_offset as f32) + self.position.x,
                    (y_offset as f32) + self.position.y,
                    self.position.z,
                ],
                color,
            });
        }

        // Index buffer
        // Create triangles by connecting the center (vertex 0) to the outer vertices
        for i in 1..=segments {
            let next_i = if i == segments { 1 } else { i + 1 };
            indices.extend_from_slice(&[0, i as u16, next_i as u16]);
        }

        (verts, indices)
    }
}