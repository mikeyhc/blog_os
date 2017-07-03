ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
            /* ensutre that the multiboot header is at the beginning */
            KEEP(*(.multiboot_header))
    }

    .text :
    {
            *(.text)
    }
}
