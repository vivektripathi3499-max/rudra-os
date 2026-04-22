use core::fmt::Write;
use spin::Mutex;

const LOG_SIZE: usize = 8192;

static LOG_BUFFER: Mutex<LogBuffer> = Mutex::new(LogBuffer {
    buf: [0; LOG_SIZE],
    pos: 0,
});

struct LogBuffer {
    buf: [u8; LOG_SIZE],
    pos: usize,
}

impl Write for LogBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            if self.pos < LOG_SIZE {
                self.buf[self.pos] = byte;
                self.pos += 1;
            }
        }
        Ok(())
    }
}

pub fn log(msg: &str) {
    let mut buffer = LOG_BUFFER.lock();
    let _ = write!(buffer, "{}", msg);
}

pub fn dump() {
    let buffer = LOG_BUFFER.lock();

    for i in 0..buffer.pos {
        crate::print!("{}", buffer.buf[i] as char);
    }

    crate::println!();
}
