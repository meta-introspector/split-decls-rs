macro_rules! exe_phdrs {
    () => {
        # [doc = " `(getauxval(AT_PHDR), getauxval(AT_PHENT), getauxval(AT_PHNUM))`—Returns"] # [doc = " the address, ELF segment header size, and number of ELF segment headers for"] # [doc = " the main executable."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/getauxval.3.html"] # [inline] pub fn exe_phdrs () -> (* const c_void , usize , usize) { backend :: param :: auxv :: exe_phdrs () }
    };
}

exe_phdrs!();