#![feature(lang_items)]
#![feature(const_fn, unique)]
#![no_std]

extern crate rlibc;
extern crate multiboot2;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    let boot_info = unsafe {
        multiboot2::load(multiboot_information_address)
    };

    vga_buffer::clear_screen();

    println!("Hello World{}", "!");

    // kernel and multiboot sizes
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("ELF-sections tag expected");
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start+ (boot_info.total_size as usize);
    println!("kernel:");
    println!("    start: 0x{:x}, length: 0x{:x}",
             kernel_start, kernel_end - kernel_start);
    println!("multiboot:");
    println!("    start: 0x{:x}, length: 0x{:x}",
             multiboot_start, multiboot_end - multiboot_start);

    // print available memory
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
                 area.base_addr, area.length);
    }
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() { }

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
                        line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}
