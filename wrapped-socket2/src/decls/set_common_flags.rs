macro_rules! deps {
    () => {
        Socket!();
    };
}

macro_rules! set_common_flags {
    () => {
        deps!();
        # [doc = " Set `FD_CLOEXEC` and `NOSIGPIPE` on the `socket` for platforms that need it."] # [inline (always)] # [allow (clippy :: unnecessary_wraps)] fn set_common_flags (socket : Socket) -> io :: Result < Socket > { # [cfg (all (unix , not (any (target_os = "android" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "fuchsia" , target_os = "hurd" , target_os = "illumos" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "espidf" , target_os = "vita" , target_os = "cygwin" ,))))] socket . _set_cloexec (true) ? ; # [cfg (any (target_os = "ios" , target_os = "visionos" , target_os = "macos" , target_os = "tvos" , target_os = "watchos" ,))] socket . _set_nosigpipe (true) ? ; Ok (socket) }
    };
}

set_common_flags!();