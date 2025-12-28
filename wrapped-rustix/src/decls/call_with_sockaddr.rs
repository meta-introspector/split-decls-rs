macro_rules! deps {
    () => {
        SocketAddrOpaque!();
        SocketAddrLen!();
    };
}

macro_rules! call_with_sockaddr {
    () => {
        deps!();
        # [doc = " Helper for implementing `SocketAddrArg::with_sockaddr`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This calls `f` with a pointer to an object it has a reference to, with the"] # [doc = " and the length of that object, so they'll be valid for the duration of the"] # [doc = " call."] pub (crate) unsafe fn call_with_sockaddr < A , R > (addr : & A , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { let ptr = as_ptr (addr) . cast () ; let len = size_of :: < A > () as SocketAddrLen ; f (ptr , len) }
    };
}

call_with_sockaddr!()