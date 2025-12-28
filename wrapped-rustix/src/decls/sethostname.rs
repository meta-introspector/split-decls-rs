macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! sethostname {
    () => {
        deps!();
        # [doc = " `sethostname(name)`—Sets the system host name."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sethostname.2.html"] # [cfg (not (any (target_os = "emscripten" , target_os = "espidf" , target_os = "horizon" , target_os = "redox" , target_os = "vita" , target_os = "wasi")))] # [inline] pub fn sethostname (name : & [u8]) -> io :: Result < () > { backend :: system :: syscalls :: sethostname (name) }
    };
}

sethostname!();