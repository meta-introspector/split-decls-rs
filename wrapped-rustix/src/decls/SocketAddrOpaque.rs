macro_rules! SocketAddrOpaque {
    () => {
        # [doc = " Opaque type equivalent to `sockaddr` in C."] # [doc = ""] # [doc = " This is always used behind a raw pointer that is cast from a pointer to a"] # [doc = " `sockaddr`-compatible C type, and then cast back to a `sockaddr` pointer to"] # [doc = " be passed to a system call."] # [repr (C)] pub struct SocketAddrOpaque { _data : [u8 ; 0] , }
    };
}

SocketAddrOpaque!()