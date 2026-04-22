use alloc::vec::Vec;
use super::framebuffer::Framebuffer;

pub struct Layer {
    pub buffer: Framebuffer,
    pub x: usize,
    pub y: usize,
    pub alpha: f32,
}

pub struct Compositor {
    pub layers: Vec<Layer>,
}

impl Compositor {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn render(&mut self, target: &mut Framebuffer) {
        for layer in &mut self.layers {
            for y in 0..layer.buffer.height {
                for x in 0..layer.buffer.width {
                    let src = layer.buffer.buffer[y * layer.buffer.width + x];
                    let dst_index = (y + layer.y) * target.width + (x + layer.x);

                    let dst = target.buffer[dst_index];

                    target.buffer[dst_index] =
                        Self::alpha_blend(src, dst, layer.alpha);
                }
            }
        }
    }

    fn alpha_blend(src: u32, dst: u32, alpha: f32) -> u32 {
        let sr = ((src >> 16) & 0xFF) as f32;
        let sg = ((src >> 8) & 0xFF) as f32;
        let sb = (src & 0xFF) as f32;

        let dr = ((dst >> 16) & 0xFF) as f32;
        let dg = ((dst >> 8) & 0xFF) as f32;
        let db = (dst & 0xFF) as f32;

        let r = sr * alpha + dr * (1.0 - alpha);
        let g = sg * alpha + dg * (1.0 - alpha);
        let b = sb * alpha + db * (1.0 - alpha);

        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }
}
