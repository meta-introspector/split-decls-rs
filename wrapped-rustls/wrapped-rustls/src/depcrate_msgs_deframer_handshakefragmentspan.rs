// Generated macro for FragmentSpan (struct)
macro_rules! Depcrate_msgs_deframer_handshakeFragmentSpan {
() => {
// Module: crate::msgs::deframer::handshake
// Provides: {"FragmentSpan"}
// Dependencies: {}
# [derive (Debug)] struct FragmentSpan { # [doc = " version taken from containing message."] version : ProtocolVersion , # [doc = " size of the handshake message body (excluding header)"] # [doc = ""] # [doc = " `None` means the size is unknown, because `bounds` is not"] # [doc = " large enough to encompass a whole header."] size : Option < usize > , # [doc = " bounds of the handshake message, including header"] bounds : Range < usize > , }
};
}
