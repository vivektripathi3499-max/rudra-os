use crate::console::CONSOLE;

/* =========================
   DRAW UI (ONLY DRAWING)
========================= */

pub fn draw_ui() {
    draw_button(300, 350, 200, 40);
}

/* =========================
   BUTTON
========================= */

pub fn draw_button(x: i32, y: i32, w: i32, h: i32) {

    if let Some(console) = CONSOLE.lock().as_mut() {

        for dy in 0..h {
            for dx in 0..w {

                let px = x + dx;
                let py = y + dy;

                console.put_pixel(px, py, 0x4444ff);
            }
        }

    }
}

use crate::fs::vfs::{root_fs, get_node_by_path, NodeType};

use spin::Mutex;

static CURRENT_PATH: Mutex<&'static str> = Mutex::new("/");

/* =========================
   FILE EXPLORER
========================= */

pub fn render_files() {
    let root = root_fs();
    let path = *CURRENT_PATH.lock();

    let current = get_node_by_path(&root, path);

    let mut y = 50;

    if let Some(console) = CONSOLE.lock().as_mut() {
    console.put_pixel(55, 55, 0xffffff); // DEBUG PIXEL (WHITE)

        // background
        for dy in 0..200 {
            for dx in 0..300 {
               console.put_pixel(50 + dx, 50 + dy, 0x0000ff); // BLUE
            }
        }

        for item in &current.children {

            let color = match item.node_type {
                NodeType::Folder => 0xffff00,
                NodeType::File => 0xffffff,
            };

            // draw item box
            for dy in 0..20 {
                for dx in 0..200 {
                    console.put_pixel(60 + dx, y + dy, color);
                }
            }

            y += 30;
        }
    }
}

pub fn handle_click(mouse_x: i32, mouse_y: i32) {
    let root = root_fs();
    let path = *CURRENT_PATH.lock();

    let current = get_node_by_path(&root, path);

    let mut y = 50;

    for item in &current.children {

        // check click inside item box
        if mouse_x >= 60 && mouse_x <= 260 &&
           mouse_y >= y && mouse_y <= y + 20 {

            match item.node_type {

                NodeType::Folder => {
                    // navigate into folder
                    let mut path_lock = CURRENT_PATH.lock();

                    if *path_lock == "/" {
                        *path_lock = item.name;
                    } else {
                        *path_lock = concat_path(*path_lock, item.name);
                    }
                }

                NodeType::File => {
                    // open file (print content)
                    if let Some(content) = item.content {
                        crate::println!("Opened {}: {}", item.name, content);
                    }
                }
            }
        }

        y += 30;
    }
}

fn concat_path(current: &str, next: &str) -> &'static str {
    match (current, next) {
        ("/", "docs") => "docs",
        ("docs", "readme.txt") => "docs/readme.txt",
        ("/", "hello.txt") => "hello.txt",

        // 🔥 FIX: NEVER return `current`
        _ => "/",
    }
}
