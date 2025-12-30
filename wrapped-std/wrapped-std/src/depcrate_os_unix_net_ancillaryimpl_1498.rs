// Generated macro for impl_1498 (impl)
macro_rules! Depcrate_os_unix_net_ancillaryimpl_1498 {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"impl_1498"}
// Dependencies: {}
impl < 'a , T > AncillaryDataIter < 'a , T > { # [doc = " Creates `AncillaryDataIter` struct to iterate through the data unit in the control message."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `data` must contain a valid control message."] unsafe fn new (data : & 'a [u8]) -> AncillaryDataIter < 'a , T > { AncillaryDataIter { data , phantom : PhantomData } } }
};
}
