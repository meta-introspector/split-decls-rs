macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! init_module {
    () => {
        deps!();
        # [doc = " `init_module`—Load a kernel module."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/init_module.2.html"] # [inline] # [cfg (linux_kernel)] pub fn init_module (image : & [u8] , param_values : & CStr) -> io :: Result < () > { backend :: system :: syscalls :: init_module (image , param_values) }
    };
}

init_module!();