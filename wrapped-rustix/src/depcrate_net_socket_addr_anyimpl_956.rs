// Generated macro for impl_956 (impl)
macro_rules! Depcrate_net_socket_addr_anyimpl_956 {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"impl_956"}
// Dependencies: {}
impl SocketAddrBuf { # [inline] pub (crate) const fn new () -> Self { Self { len : size_of :: < SocketAddrStorage > () as c :: socklen_t , storage : MaybeUninit :: < SocketAddrStorage > :: uninit () , } } # [doc = " Convert the buffer into [`SocketAddrAny`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " A valid address must have been written into `self.storage` and its"] # [doc = " length written into `self.len`."] # [inline] pub (crate) unsafe fn into_any (self) -> SocketAddrAny { SocketAddrAny :: new (self . storage , bitcast ! (self . len)) } # [doc = " Convert the buffer into [`Option<SocketAddrAny>`]."] # [doc = ""] # [doc = " This returns `None` if `len` is zero or other platform-specific"] # [doc = " conditions define the address as empty."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Either valid address must have been written into `self.storage` and its"] # [doc = " length written into `self.len`, or `self.len` must have been set to 0."] # [inline] pub (crate) unsafe fn into_any_option (self) -> Option < SocketAddrAny > { let len = bitcast ! (self . len) ; if read_sockaddr :: sockaddr_nonempty (self . storage . as_ptr () . cast () , len) { Some (SocketAddrAny :: new (self . storage , len)) } else { None } } }
};
}
