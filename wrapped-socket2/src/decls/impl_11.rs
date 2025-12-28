macro_rules! deps {
    () => {
        SockAddr!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < SocketAddrV4 > for SockAddr { fn from (addr : SocketAddrV4) -> SockAddr { let mut storage = unsafe { mem :: zeroed :: < sockaddr_storage > () } ; let len = { let storage = unsafe { & mut * ptr :: addr_of_mut ! (storage) . cast :: < sockaddr_in > () } ; storage . sin_family = AF_INET as sa_family_t ; storage . sin_port = addr . port () . to_be () ; storage . sin_addr = crate :: sys :: to_in_addr (addr . ip ()) ; storage . sin_zero = Default :: default () ; mem :: size_of :: < sockaddr_in > () as socklen_t } ; # [cfg (any (target_os = "dragonfly" , target_os = "freebsd" , target_os = "haiku" , target_os = "hermit" , target_os = "ios" , target_os = "visionos" , target_os = "macos" , target_os = "netbsd" , target_os = "nto" , target_os = "openbsd" , target_os = "tvos" , target_os = "vxworks" , target_os = "watchos" ,))] { storage . ss_len = len as u8 ; } SockAddr { storage , len } } }
    };
}

impl_11!()