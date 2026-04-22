use bootloader_api::info::FrameBufferInfo;
use core::fmt::{self, Write};
use font8x8::UnicodeFonts;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONSOLE: Mutex<Option<Console>> = Mutex::new(None);
}

const CHAR_SPACING_X: usize = 10;
const CHAR_SPACING_Y: usize = 20;



pub struct Console {
    pub buffer: &'static mut [u8],
    pub info: FrameBufferInfo,
    pub x: usize,
    pub y: usize,
    pub height: usize,
}


impl Console {

    // =============================
    // BASIC
    // =============================
    pub fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Self {
            buffer,
            height: info.height,
            info,
            x: 0,
            y: 0,
        }
    }

    pub fn init_console(buffer: &'static mut [u8], info: FrameBufferInfo) {
        let console = Console::new(buffer, info);
        *CONSOLE.lock() = Some(console);
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    // =============================
    // CLEARING
    // =============================
   pub fn clear(&mut self) {

    // 🔥 DO NOT CLEAR IN GUI MODE
    if crate::MODE.load(core::sync::atomic::Ordering::Relaxed) == 1 {
        return;
    }

    for pixel in self.buffer.iter_mut() {
        *pixel = 0;
    }

    self.x = 0;
    self.y = 0;
}

    pub fn clear_region(&mut self, x: usize, y: usize, w: usize, h: usize) {

    // 🔥 DO NOT CLEAR IN GUI MODE
    if crate::MODE.load(core::sync::atomic::Ordering::Relaxed) == 1 {
        return;
    }

    for py in y..(y + h) {
        for px in x..(x + w) {

            let offset =
                (py * self.info.stride + px)
                * self.info.bytes_per_pixel;

            if offset + self.info.bytes_per_pixel <= self.buffer.len() {
                self.buffer[offset] = 0;
                self.buffer[offset + 1] = 0;
                self.buffer[offset + 2] = 0;
            }
        }
    }
}

    // =============================
    // DRAWING
    // =============================
  pub fn write_char(&mut self, c: char) {

    if crate::MODE.load(core::sync::atomic::Ordering::Relaxed) == 1 {
        return;
    }

    let bpp = self.info.bytes_per_pixel;

    // clear char cell
    for row in 0..CHAR_SPACING_Y {
        for col in 0..CHAR_SPACING_X {

            let px = self.x + col;
            let py = self.y + row;

            if px >= self.info.width || py >= self.info.height {
                continue;
            }

            let offset = (py * self.info.stride + px) * bpp;

            if offset + 3 >= self.buffer.len() {
                continue;
            }

            self.buffer[offset] = 0;
            self.buffer[offset + 1] = 0;
            self.buffer[offset + 2] = 0;
        }
    }

    // draw glyph
    if let Some(glyph) = font8x8::BASIC_FONTS.get(c) {
        for (row, byte) in glyph.iter().enumerate() {
            for col in 0..8 {
                if (byte >> col) & 1 == 1 {

                    let px = self.x + col;
                    let py = self.y + row;

                    if px >= self.info.width || py >= self.info.height {
                        continue;
                    }

                    let offset = (py * self.info.stride + px) * bpp;

                    if offset + 3 >= self.buffer.len() {
                        continue;
                    }

                    self.buffer[offset] = 255;
                    self.buffer[offset + 1] = 255;
                    self.buffer[offset + 2] = 255;
                }
            }
        }
    }

    self.x += CHAR_SPACING_X;
}

  pub fn newline(&mut self) {

    if crate::MODE.load(core::sync::atomic::Ordering::Relaxed) == 1 {
        return;
    }

    self.y += CHAR_SPACING_Y;
    self.x = 0;

    if self.y + CHAR_SPACING_Y >= self.height {
        self.scroll();
    }
}

    fn scroll(&mut self) {

        let stride = self.info.stride;
        let bpp = self.info.bytes_per_pixel;
        let line_height = CHAR_SPACING_Y;

        let row_bytes = stride * bpp * line_height;
        let total = self.buffer.len();

        for i in 0..(total - row_bytes) {
            self.buffer[i] = self.buffer[i + row_bytes];
        }

        for i in (total - row_bytes)..total {
            self.buffer[i] = 0;
        }

        self.y -= line_height;
    }

   pub fn backspace(&mut self) {

    if crate::MODE.load(core::sync::atomic::Ordering::Relaxed) == 1 {
        return;
    }

    if self.x < CHAR_SPACING_X {
        return;
    }

    self.x -= CHAR_SPACING_X;

    for row in 0..CHAR_SPACING_Y {
        for col in 0..CHAR_SPACING_X {

            let px = self.x + col;
            let py = self.y + row;

            let offset =
                (py * self.info.stride + px)
                * self.info.bytes_per_pixel;

          let bpp = self.info.bytes_per_pixel;

if offset + bpp <= self.buffer.len() {
    for b in 0..bpp {
        self.buffer[offset + b] = 0;
    }
}
        }
    }
}

   pub fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
    if x < 0 || y < 0 {
        return;
    }

    let x = x as usize;
    let y = y as usize;

    if x >= self.info.width || y >= self.info.height {
        return;
    }

    let bpp = self.info.bytes_per_pixel;
    let offset = (y * self.info.stride + x) * bpp;

    // 🔥 CRITICAL FIX: FULL BOUNDS CHECK
    if offset + 3 >= self.buffer.len() {
        return;
    }

    let r = ((color >> 16) & 0xff) as u8;
    let g = ((color >> 8) & 0xff) as u8;
    let b = (color & 0xff) as u8;

    self.buffer[offset] = b;
    self.buffer[offset + 1] = g;
    self.buffer[offset + 2] = r;

    // 🔥 ALSO CLEAR ALPHA (IMPORTANT FOR SOME FBs)
    if bpp == 4 {
        self.buffer[offset + 3] = 0;
    }
}
}

// =============================
// WRITE TRAIT
// =============================
impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            match c {
                '\n' => self.newline(),
                _ => self.write_char(c), // 👈 Update this line
            }
        }
        Ok(())
    }
}


// =============================
// GLOBAL HELPERS
// =============================
pub fn clear_screen() {
    if let Some(console) = CONSOLE.lock().as_mut() {
        console.clear();
    }
}
