macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! finit_module {
    () => {
        deps!();
        # [doc = " `finit_module`—Load a kernel module from a file descriptor."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/finit_module.2.html"] # [inline] # [cfg (linux_kernel)] pub fn finit_module < Fd : AsFd > (fd : Fd , param_values : & CStr , flags : c_int) -> io :: Result < () > { backend :: system :: syscalls :: finit_module (fd . as_fd () , param_values , flags) }
    };
}

finit_module!()