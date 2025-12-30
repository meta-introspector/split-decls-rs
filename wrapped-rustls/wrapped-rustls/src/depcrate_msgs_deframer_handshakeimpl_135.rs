// Generated macro for impl_135 (impl)
macro_rules! Depcrate_msgs_deframer_handshakeimpl_135 {
() => {
// Module: crate::msgs::deframer::handshake
// Provides: {"impl_135"}
// Dependencies: {}
impl Iterator for DissectHandshakeIter < '_ , '_ > { type Item = FragmentSpan ; fn next (& mut self) -> Option < Self :: Item > { if self . payload . is_empty () { return None ; } if self . payload . len () < HANDSHAKE_HEADER_LEN { let buf = mem :: take (& mut self . payload) ; let bounds = self . containing_buffer . locate (buf) ; return Some (FragmentSpan { version : self . version , size : None , bounds : bounds . clone () , }) ; } let (header , rest) = mem :: take (& mut self . payload) . split_at (HANDSHAKE_HEADER_LEN) ; let size = u24 :: read_bytes (& header [1 ..]) . unwrap () . into () ; let available = if size < rest . len () { self . payload = & rest [size ..] ; size } else { rest . len () } ; let mut bounds = self . containing_buffer . locate (header) ; bounds . end += available ; Some (FragmentSpan { version : self . version , size : Some (size) , bounds : bounds . clone () , }) } }
};
}
