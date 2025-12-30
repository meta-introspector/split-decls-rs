// Generated macro for impl_911 (impl)
macro_rules! Depcrate_net_send_recv_msgimpl_911 {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"impl_911"}
// Dependencies: {}
impl < 'data , T > AncillaryIter < 'data , T > { # [doc = " Create a new iterator over data in an ancillary buffer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The buffer must contain valid ancillary data."] unsafe fn new (data : & 'data mut [u8]) -> Self { assert_eq ! (data . len () % size_of ::< T > () , 0) ; Self { data , _marker : PhantomData , } } }
};
}
