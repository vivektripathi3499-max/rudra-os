use crate::console::CONSOLE;
use spin::Mutex;
use alloc::vec::Vec;

#[derive(Clone)]
pub struct Window {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub dragging: bool,
    pub drag_offset_x: i32,
    pub drag_offset_y: i32,
    pub focused: bool,
}

static WINDOWS: Mutex<Vec<Window>> = Mutex::new(Vec::new());

/* =========================
   WINDOW CREATION
========================= */

pub fn create_window(x: i32, y: i32, w: i32, h: i32) {
    let mut windows = WINDOWS.lock();

    let id = windows.len() as u32;

    windows.push(Window {
        id,
        x,
        y,
        width: w,
        height: h,
        dragging: false,
        drag_offset_x: 0,
        drag_offset_y: 0,
        focused: true,
    });
}

/* =========================
   DRAWING
========================= */

pub fn draw_all_windows() {
    if let Some(console) = CONSOLE.lock().as_mut() {

console.put_pixel(200, 200, 0xff0000);

        let windows = WINDOWS.lock();

        for win in windows.iter() {

            // --- Window background ---
            for dy in 0..win.height {
                for dx in 0..win.width {

                    let px = win.x + dx;
                    let py = win.y + dy;

                    console.put_pixel(px, py, 0x00ff00);
                }
            }

            // --- Title bar ---
           let title_color = if win.focused { 0xff0000 } else { 0xaa0000 };

            for dy in 0..20 {
                for dx in 0..win.width {

                   let px = win.x + dx;
let py = win.y + dy;

                    console.put_pixel(px, py, title_color);
                }
            }

            // --- Simple border ---
            for dx in 0..win.width {
                console.put_pixel(win.x + dx, win.y, 0xffffff);
                console.put_pixel(win.x + dx, win.y + win.height - 1, 0xffffff);
            }

            for dy in 0..win.height {
                console.put_pixel(win.x, win.y + dy, 0xffffff);
                console.put_pixel(win.x + win.width - 1, win.y + dy, 0xffffff);
            }
        }
    }
}

/* =========================
   REDRAW
========================= */
pub fn redraw() {
    if let Some(console) = crate::console::CONSOLE.lock().as_mut() {

        // CLEAR FIRST
        for y in 0..300 {
            for x in 0..400 {
                console.put_pixel(x as i32, y as i32, 0x000000);
            }
        }

        // TEST PIXELS
        console.put_pixel(10, 10, 0xffffff);

      draw_all_windows();
//crate::ui::render_files();
    }
}
/* =========================
   MOUSE INPUT
========================= */

pub fn mouse_down(mx: i32, my: i32) {
    let mut windows = WINDOWS.lock();

    let mut clicked_index: Option<usize> = None;

    // Check window title bar click
    for i in (0..windows.len()).rev() {
        let win = &windows[i];

        if mx >= win.x && mx <= win.x + win.width &&
           my >= win.y && my <= win.y + 20 {

            clicked_index = Some(i);
            break;
        }
    }

    if let Some(i) = clicked_index {

        // focus window
        for w in windows.iter_mut() {
            w.focused = false;
        }

        let win = &mut windows[i];

        win.focused = true;
        win.dragging = true;
        win.drag_offset_x = mx - win.x;
        win.drag_offset_y = my - win.y;

    } else {
        // 🔥 IMPORTANT: PASS CLICK TO UI
        crate::ui::handle_click(mx, my);
    }
}
pub fn mouse_move(mx: i32, my: i32) {
    let mut windows = WINDOWS.lock();

    for win in windows.iter_mut() {
        if win.dragging {
            win.x = mx - win.drag_offset_x;
            win.y = my - win.drag_offset_y;
        }
    }
}

pub fn mouse_up() {
    let mut windows = WINDOWS.lock();

    for win in windows.iter_mut() {
        win.dragging = false;
    }
}

/* =========================
   OPTIONAL: WINDOW UTILITIES
========================= */

pub fn bring_to_front(id: u32) {
    let mut windows = WINDOWS.lock();

    if let Some(pos) = windows.iter().position(|w| w.id == id) {
        let win = windows.remove(pos);
        windows.push(win);
    }
}

pub fn close_window(id: u32) {
    let mut windows = WINDOWS.lock();
    windows.retain(|w| w.id != id);
}
