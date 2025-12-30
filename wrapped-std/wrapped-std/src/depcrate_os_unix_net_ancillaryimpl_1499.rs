// Generated macro for impl_1499 (impl)
macro_rules! Depcrate_os_unix_net_ancillaryimpl_1499 {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"impl_1499"}
// Dependencies: {}
impl < 'a , T > Iterator for AncillaryDataIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { if size_of :: < T > () <= self . data . len () { unsafe { let unit = read_unaligned (self . data . as_ptr () . cast ()) ; self . data = & self . data [size_of :: < T > () ..] ; Some (unit) } } else { None } } }
};
}
