use crate::console::CONSOLE;
use spin::Mutex;

static CURSOR_X: Mutex<i32> = Mutex::new(400);
static CURSOR_Y: Mutex<i32> = Mutex::new(300);

static PREV_X: Mutex<i32> = Mutex::new(400);
static PREV_Y: Mutex<i32> = Mutex::new(300);

pub fn update_cursor(x: i32, y: i32) {

    let mut cx = CURSOR_X.lock();
    let mut cy = CURSOR_Y.lock();

    let mut px = PREV_X.lock();
    let mut py = PREV_Y.lock();

    clear_cursor(*px, *py);

    *cx = x;
    *cy = y;

    draw_cursor(x, y);

    *px = x;
    *py = y;
}

pub fn draw_cursor(x: i32, y: i32) {
    if let Some(console) = CONSOLE.lock().as_mut() {
        for i in 0..8 {
            console.put_pixel(x + i, y, 0xffffff);
            console.put_pixel(x, y + i, 0xffffff);
        }
    }
}

fn clear_cursor(x: i32, y: i32) {
    if let Some(console) = CONSOLE.lock().as_mut() {
        for i in 0..8 {
            console.put_pixel(x + i, y, 0x000000);
            console.put_pixel(x, y + i, 0x000000);
        }
    }
}
