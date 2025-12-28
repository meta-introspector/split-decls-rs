macro_rules! macro_167 {
    () => {
        cfg_not_wasi ! { # [cfg (feature = "net")] pub (crate) use addr :: to_socket_addrs ; }
    };
}

macro_167!();