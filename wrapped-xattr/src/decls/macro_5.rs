macro_rules! macro_5 {
    () => {
        platforms ! { "android" ; "linux" ; "macos" ; "hurd" => linux_macos , "freebsd" ; "netbsd" => bsd }
    };
}

macro_5!()