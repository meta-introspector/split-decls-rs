// Generated macro for impl_916 (impl)
macro_rules! Depcrate_net_send_recv_msgimpl_916 {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"impl_916"}
// Dependencies: {}
impl < T > DoubleEndedIterator for AncillaryIter < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . data . len () < size_of :: < T > () { return None ; } let item = unsafe { let ptr = self . data . as_ptr () . add (self . data . len () - size_of :: < T > ()) ; ptr . cast :: < T > () . read_unaligned () } ; let len = self . data . len () ; let data = take (& mut self . data) ; self . data = & mut data [.. len - size_of :: < T > ()] ; Some (item) } }
};
}
