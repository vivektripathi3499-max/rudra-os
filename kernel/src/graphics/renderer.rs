use super::framebuffer::Framebuffer;

pub struct Renderer;

impl Renderer {
    pub fn draw_rounded_rect(
        fb: &mut Framebuffer,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        radius: usize,
        color: u32,
    ) {
        for iy in 0..h {
            for ix in 0..w {
                let dx = if ix < radius {
                    radius - ix
                } else if ix > w - radius {
                    ix - (w - radius)
                } else {
                    0
                };

                let dy = if iy < radius {
                    radius - iy
                } else if iy > h - radius {
                    iy - (h - radius)
                } else {
                    0
                };

                if dx * dx + dy * dy <= radius * radius {
                    fb.draw_pixel(x + ix, y + iy, color);
                }
            }
        }
    }
}
