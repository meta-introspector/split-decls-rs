macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! delete_module {
    () => {
        deps!();
        # [doc = " `delete_module`—Unload a kernel module."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/delete_module.2.html"] # [inline] # [cfg (linux_kernel)] pub fn delete_module (name : & CStr , flags : c_int) -> io :: Result < () > { backend :: system :: syscalls :: delete_module (name , flags) }
    };
}

delete_module!()