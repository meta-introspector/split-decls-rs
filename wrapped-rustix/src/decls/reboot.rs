macro_rules! deps {
    () => {
        RebootCommand!();
        Result!();
    };
}

macro_rules! reboot {
    () => {
        deps!();
        # [doc = " `reboot`—Reboot the system or enable/disable Ctrl-Alt-Del."] # [doc = ""] # [doc = " The reboot syscall, despite the name, can actually do much more than"] # [doc = " reboot."] # [doc = ""] # [doc = " Among other things, it can:"] # [doc = "  - Restart, Halt, Power Off, and Suspend the system"] # [doc = "  - Enable and disable the Ctrl-Alt-Del keystroke"] # [doc = "  - Execute other kernels"] # [doc = "  - Terminate init inside PID namespaces"] # [doc = ""] # [doc = " It is highly recommended to carefully read the kernel documentation before"] # [doc = " calling this function."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/reboot.2.html"] # [cfg (target_os = "linux")] pub fn reboot (cmd : RebootCommand) -> io :: Result < () > { backend :: system :: syscalls :: reboot (cmd) }
    };
}

reboot!()