macro_rules! deps {
    () => {
        SendAncillaryBuffer!();
        SocketAddrAny!();
        MMsgHdr!();
        SocketAddrArg!();
    };
}

macro_rules! impl_585 {
    () => {
        deps!();
        # [cfg (target_os = "linux")] impl < 'a > MMsgHdr < 'a > { # [doc = " Constructs a new message with no destination address."] pub fn new (iov : & 'a [IoSlice < '_ >] , control : & 'a mut SendAncillaryBuffer < '_ , '_ , '_ >) -> Self { Self :: wrap (noaddr_msghdr (iov , control)) } # [doc = " Constructs a new message to a specific address."] # [doc = ""] # [doc = " This requires a `SocketAddrAny` instead of using `impl SocketAddrArg`;"] # [doc = " to obtain a `SocketAddrAny`, use [`SocketAddrArg::as_any`]."] pub fn new_with_addr (addr : & 'a SocketAddrAny , iov : & 'a [IoSlice < '_ >] , control : & 'a mut SendAncillaryBuffer < '_ , '_ , '_ > ,) -> Self { let mut msghdr = noaddr_msghdr (iov , control) ; msghdr . msg_name = addr . as_ptr () as _ ; msghdr . msg_namelen = bitcast ! (addr . addr_len ()) ; Self :: wrap (msghdr) } fn wrap (msg_hdr : c :: msghdr) -> Self { Self { raw : c :: mmsghdr { msg_hdr , msg_len : 0 , } , _phantom : PhantomData , } } # [doc = " Returns the number of bytes sent. This will return 0 until after a"] # [doc = " successful call to [sendmmsg]."] pub fn bytes_sent (& self) -> usize { self . raw . msg_len as usize } }
    };
}

impl_585!();