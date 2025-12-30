// Generated macro for impl_140 (impl)
macro_rules! Depcrate_msgs_deframer_handshakeimpl_140 {
() => {
// Module: crate::msgs::deframer::handshake
// Provides: {"impl_140"}
// Dependencies: {}
impl FragmentSpan { # [doc = " A `FragmentSpan` is \"complete\" if its size is known, and its"] # [doc = " bounds exactly encompasses one handshake message."] fn is_complete (& self) -> bool { match self . size { Some (sz) => sz + HANDSHAKE_HEADER_LEN == self . bounds . len () , None => false , } } }
};
}
