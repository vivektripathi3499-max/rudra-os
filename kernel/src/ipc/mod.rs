use crate::println;

#[derive(Copy, Clone)]
pub struct Message {
    pub from: usize,
    pub to: usize,
    pub data: [u8; 64],
}

static mut MESSAGES: [Option<Message>; 32] = [None; 32];

pub fn send(from: usize, to: usize, text: &str) {

    unsafe {

        for slot in MESSAGES.iter_mut() {

            if slot.is_none() {

                let mut data = [0u8; 64];

                for (i, b) in text.bytes().enumerate() {
                    if i < 64 {
                        data[i] = b;
                    }
                }

                *slot = Some(Message {
                    from,
                    to,
                    data,
                });

                println!("[ipc] message sent {} -> {}", from, to);
                return;
            }
        }

        println!("[ipc] message queue full");

    }

}

pub fn receive(pid: usize) {

    unsafe {

        for slot in MESSAGES.iter_mut() {

            if let Some(msg) = slot {

                if msg.to == pid {

                    let text = core::str::from_utf8(&msg.data).unwrap_or("");

                    println!("[ipc] from {} : {}", msg.from, text);

                    *slot = None;

                    return;
                }
            }
        }

        println!("[ipc] no messages");

    }

}
