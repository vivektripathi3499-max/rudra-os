pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: &'static mut [u32], // ARGB
}

impl Framebuffer {
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = y * self.width + x;
        self.buffer[index] = color;
    }

    pub fn clear(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for iy in y..(y + h) {
            for ix in x..(x + w) {
                self.draw_pixel(ix, iy, color);
            }
        }
    }
}
