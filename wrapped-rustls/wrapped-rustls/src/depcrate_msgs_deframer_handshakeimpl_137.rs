// Generated macro for impl_137 (impl)
macro_rules! Depcrate_msgs_deframer_handshakeimpl_137 {
() => {
// Module: crate::msgs::deframer::handshake
// Provides: {"impl_137"}
// Dependencies: {}
impl < 'b > Iterator for HandshakeIter < '_ , 'b > { type Item = (InboundPlainMessage < 'b > , usize) ; fn next (& mut self) -> Option < Self :: Item > { let next_span = self . deframer . spans . get (self . index) ? ; if ! next_span . is_complete () { return None ; } let discard = if self . deframer . spans . len () - 1 == self . index { mem :: take (& mut self . deframer . outer_discard) } else { 0 } ; self . index += 1 ; Some ((InboundPlainMessage { typ : ContentType :: Handshake , version : next_span . version , payload : self . containing_buffer . slice_from_range (& next_span . bounds) , } , discard ,)) } }
};
}
