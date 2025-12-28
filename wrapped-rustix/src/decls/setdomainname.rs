macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! setdomainname {
    () => {
        deps!();
        # [doc = " `setdomain(name)`—Sets the system NIS domain name."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [FreeBSD]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setdomainname.2.html"] # [doc = " [FreeBSD]: https://man.freebsd.org/cgi/man.cgi?query=setdomainname&sektion=3"] # [cfg (not (any (target_os = "cygwin" , target_os = "emscripten" , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "illumos" , target_os = "redox" , target_os = "solaris" , target_os = "vita" , target_os = "wasi" ,)))] # [inline] pub fn setdomainname (name : & [u8]) -> io :: Result < () > { backend :: system :: syscalls :: setdomainname (name) }
    };
}

setdomainname!()