// Generated macro for impl_146 (impl)
macro_rules! Depcrate_msgs_deframerimpl_146 {
() => {
// Module: crate::msgs::deframer
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'a > Iterator for DeframerIter < 'a > { type Item = Result < InboundOpaqueMessage < 'a > , Error > ; fn next (& mut self) -> Option < Self :: Item > { let mut reader = Reader :: init (self . buf) ; let (typ , version , len) = match read_opaque_message_header (& mut reader) { Ok (header) => header , Err (err) => { let err = match err { MessageError :: TooShortForHeader | MessageError :: TooShortForLength => { return None ; } MessageError :: InvalidEmptyPayload => InvalidMessage :: InvalidEmptyPayload , MessageError :: MessageTooLarge => InvalidMessage :: MessageTooLarge , MessageError :: InvalidContentType => InvalidMessage :: InvalidContentType , MessageError :: UnknownProtocolVersion => InvalidMessage :: UnknownProtocolVersion , } ; return Some (Err (err . into ())) ; } } ; let end = HEADER_SIZE + len as usize ; self . buf . get (HEADER_SIZE .. end) ? ; let (consumed , remainder) = mem :: take (& mut self . buf) . split_at_mut (end) ; self . buf = remainder ; self . consumed += end ; Some (Ok (InboundOpaqueMessage :: new (typ , version , & mut consumed [HEADER_SIZE ..] ,))) } }
};
}
