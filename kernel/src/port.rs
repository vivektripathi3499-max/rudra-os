pub unsafe fn inb(port: u16) -> u8 {

    let value: u8;

    core::arch::asm!(
        "in al, dx",
        in("dx") port,
        out("al") value
    );

    value
}

pub unsafe fn outb(port: u16, value: u8) {

    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value
    );
}
