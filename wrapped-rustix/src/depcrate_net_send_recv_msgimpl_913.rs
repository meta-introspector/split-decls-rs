// Generated macro for impl_913 (impl)
macro_rules! Depcrate_net_send_recv_msgimpl_913 {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"impl_913"}
// Dependencies: {}
impl < T > Iterator for AncillaryIter < '_ , T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . data . len () < size_of :: < T > () { return None ; } let item = unsafe { self . data . as_ptr () . cast :: < T > () . read_unaligned () } ; let data = take (& mut self . data) ; self . data = & mut data [size_of :: < T > () ..] ; Some (item) } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } fn count (self) -> usize { self . len () } fn last (mut self) -> Option < Self :: Item > { self . next_back () } }
};
}
