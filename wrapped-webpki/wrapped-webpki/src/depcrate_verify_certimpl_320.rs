// Generated macro for impl_320 (impl)
macro_rules! Depcrate_verify_certimpl_320 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_320"}
// Dependencies: {}
impl DoubleEndedIterator for IntermediateIterator < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { match self . intermediates . split_last () { Some ((head , tail)) => { self . intermediates = tail ; Some (head . as_ref () . unwrap ()) } None => None , } } }
};
}
