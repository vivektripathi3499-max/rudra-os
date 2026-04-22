use crate::console::CONSOLE;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn short_delay() {
    for _ in 0..2_000 {
        core::hint::spin_loop();
    }
}

pub fn delay_ms(ms: u64) {
    let loops = ms * 1000;

    for _ in 0..loops {
        core::hint::spin_loop();
    }
}

static IMAGE: &[u8] = include_bytes!("assets/rudra.raw");

pub fn show() {

    let mut console_lock = CONSOLE.lock();
    let console = console_lock.as_mut().unwrap();

    let buffer = &mut console.buffer;
    let info = console.info;

    let stride = info.stride;
    let bpp = info.bytes_per_pixel;

    let screen_w = info.width;
    let screen_h = info.height;

    // calculate center position
    let offset_x = (screen_w.saturating_sub(WIDTH)) / 2;
    let offset_y = (screen_h.saturating_sub(HEIGHT)) / 2;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {

            let img_i = (y * WIDTH + x) * 3;

            let r = IMAGE[img_i];
            let g = IMAGE[img_i + 1];
            let b = IMAGE[img_i + 2];

            let fb_i = ((y + offset_y) * stride + (x + offset_x)) * bpp;

            // protect against framebuffer overflow
            if fb_i + 2 < buffer.len() {
                buffer[fb_i] = b;
                buffer[fb_i + 1] = g;
                buffer[fb_i + 2] = r;
            }
        }
    }
}

pub fn loading_bar() {

    let (buffer_ptr, info);

    {
        let mut console_lock = CONSOLE.lock();
        let console = console_lock.as_mut().unwrap();

        buffer_ptr = console.buffer.as_mut_ptr();
        info = console.info;
    }

    let stride = info.stride;
    let bpp = info.bytes_per_pixel;

    let screen_w = info.width;
    let screen_h = info.height;

    let bar_width = 200;
    let bar_height = 10;

    let start_x = (screen_w - bar_width) / 2;
    let start_y = (screen_h / 2) + 120;

    unsafe {

        let buffer = core::slice::from_raw_parts_mut(
            buffer_ptr,
            stride * screen_h * bpp
        );

for progress in 0..bar_width {

    // clear previous frame
    for y in 0..bar_height {
        for x in 0..bar_width {

            let fb_i = ((start_y + y) * stride + (start_x + x)) * bpp;

            if fb_i + 2 < buffer.len() {
                buffer[fb_i] = 0;
                buffer[fb_i + 1] = 0;
                buffer[fb_i + 2] = 0;
            }
        }
    }

            short_delay();
        }
    }
}
