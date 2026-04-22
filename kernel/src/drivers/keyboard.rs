use spin::Mutex;

use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use pc_keyboard::DecodedKey;

static KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
    Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    ));

static INPUT_BUFFER: Mutex<[u8; 256]> = Mutex::new([0; 256]);
static INPUT_LEN: Mutex<usize> = Mutex::new(0);

pub enum KeyOutput {
    Char(char),
    Backspace,
}

pub fn handle_scancode(scancode: u8) -> Option<KeyOutput> {

    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(event)) = keyboard.add_byte(scancode) {

        if let Some(key) = keyboard.process_keyevent(event) {

            match key {

                DecodedKey::Unicode('\u{8}') => {
                    return Some(KeyOutput::Backspace);
                }

DecodedKey::Unicode(c) => {
    return Some(KeyOutput::Char(c));
}


                _ => {}
            }

        }

    }

    None
}

pub fn handle_interrupt(scancode: u8) {
    if let Some(key) = handle_scancode(scancode) {
        match key {
            KeyOutput::Char(c) => {
                // Send character directly to shell
                crate::shell::handle_key(c);
            }
            KeyOutput::Backspace => {
                // Send backspace to shell
                crate::shell::handle_key('\u{8}');
                // Visually erase the character
                crate::print::backspace();
            }
        }
    }
}
