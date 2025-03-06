use core::arch::asm;

const COM1: u16 = 0x3F8;

pub unsafe fn puts(s: &[u8]) {
    for c in s {
        putchar(*c);
    }
}

pub unsafe fn putchar(c: u8) {
    outb(COM1, c);
}

pub unsafe fn init_serial() {
    // https://wiki.osdev.org/Serial_Ports#Example_Code

    outb(COM1 + 1, 0x00); // Disable all interrupts
    outb(COM1 + 3, 0x80); // Enable DLAB (set baud rate divisor)
    outb(COM1 + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
    outb(COM1 + 1, 0x00); //                  (hi byte)
    outb(COM1 + 3, 0x03); // 8 bits, no parity, one stop bit
    outb(COM1 + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
    outb(COM1 + 4, 0x0B); // IRQs enabled, RTS/DSR set
    outb(COM1 + 4, 0x1E); // Set in loopback mode, test the serial chip
    outb(COM1 + 0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

    // Check if serial is faulty (i.e: not same byte as sent)
    if (inb(COM1 + 0) != 0xAE) {
        return;
    }

    // If serial is not faulty set it in normal operation mode
    // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
    outb(COM1 + 4, 0x0F);
}

pub unsafe fn outb(port: u16, value: u8) {
    asm! {
        "out dx, al",
        in("dx") port,
        in("al") value,
    }
}

pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm! {
        "in al, dx",
        in("dx") port,
        out("al") value,
    }
    value
}
