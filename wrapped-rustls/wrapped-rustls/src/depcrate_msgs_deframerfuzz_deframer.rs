// Generated macro for fuzz_deframer (function)
macro_rules! Depcrate_msgs_deframerfuzz_deframer {
() => {
// Module: crate::msgs::deframer
// Provides: {"fuzz_deframer"}
// Dependencies: {}
pub fn fuzz_deframer (data : & [u8]) { let mut buf = data . to_vec () ; let mut iter = DeframerIter :: new (& mut buf) ; for message in iter . by_ref () { if message . is_err () { break ; } } assert ! (iter . bytes_consumed () <= buf . len ()) ; }
};
}
