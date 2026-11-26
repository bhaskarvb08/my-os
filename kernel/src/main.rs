#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();

    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    buffer.fill(0);

    draw_text(buffer, &info, "Hello, World!", 40, 40);

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

fn glyph(c: char) -> [u8; 7] {
    match c {
        'H' => [
            0b10001,
            0b10001,
            0b10001,
            0b11111,
            0b10001,
            0b10001,
            0b10001,
        ],
        'e' => [
            0b00000,
            0b01110,
            0b10001,
            0b11111,
            0b10000,
            0b10001,
            0b01110,
        ],
        'l' => [
            0b01100,
            0b00100,
            0b00100,
            0b00100,
            0b00100,
            0b00100,
            0b01110,
        ],
        'o' => [
            0b00000,
            0b01110,
            0b10001,
            0b10001,
            0b10001,
            0b10001,
            0b01110,
        ],
        ',' => [
            0b00000,
            0b00000,
            0b00000,
            0b00000,
            0b00110,
            0b00100,
            0b01000,
        ],
        'W' => [
            0b10001,
            0b10001,
            0b10001,
            0b10101,
            0b10101,
            0b10101,
            0b01010,
        ],
        'r' => [
            0b00000,
            0b10110,
            0b11001,
            0b10000,
            0b10000,
            0b10000,
            0b10000,
        ],
        'd' => [
            0b00001,
            0b00001,
            0b01101,
            0b10011,
            0b10001,
            0b10011,
            0b01101,
        ],
        '!' => [
            0b00100,
            0b00100,
            0b00100,
            0b00100,
            0b00100,
            0b00000,
            0b00100,
        ],
        _ => [0; 7],
    }
}

fn draw_pixel(
    buffer: &mut [u8],
    info: &bootloader_api::info::FrameBufferInfo,
    x: usize,
    y: usize,
) {
    if x >= info.width || y >= info.height {
        return;
    }

    let offset = (y * info.stride + x) * info.bytes_per_pixel;

    buffer[offset] = 255;
    buffer[offset + 1] = 255;
    buffer[offset + 2] = 255;
}

fn draw_text(
    buffer: &mut [u8],
    info: &bootloader_api::info::FrameBufferInfo,
    text: &str,
    start_x: usize,
    start_y: usize,
) {
    let scale = 4;

    for (char_index, c) in text.chars().enumerate() {
        let bitmap = glyph(c);

        for (row, bits) in bitmap.iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            draw_pixel(
                                buffer,
                                info,
                                start_x + char_index * 6 * scale + col * scale + dx,
                                start_y + row * scale + dy,
                            );
                        }
                    }
                }
            }
        }
    }
}