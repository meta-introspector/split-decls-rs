// Generated macro for HandshakeDeframer (struct)
macro_rules! Depcrate_msgs_deframer_handshakeHandshakeDeframer {
() => {
// Module: crate::msgs::deframer::handshake
// Provides: {"HandshakeDeframer"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct HandshakeDeframer { # [doc = " Spans covering individual handshake payloads, in order of receipt."] spans : Vec < FragmentSpan > , # [doc = " Discard value, tracking the rightmost extent of the last message"] # [doc = " in `spans`."] outer_discard : usize , }
};
}
