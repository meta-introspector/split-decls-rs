// Generated macro for impl_319 (impl)
macro_rules! Depcrate_verify_certimpl_319 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_319"}
// Dependencies: {}
impl < 'a > Iterator for IntermediateIterator < 'a > { type Item = & 'a Cert < 'a > ; fn next (& mut self) -> Option < Self :: Item > { match self . intermediates . split_first () { Some ((head , tail)) => { self . intermediates = tail ; Some (head . as_ref () . unwrap ()) } None => None , } } }
};
}
