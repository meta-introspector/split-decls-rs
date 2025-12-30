// Generated macro for read_opaque_message_header (function)
macro_rules! Depcrate_msgs_messageread_opaque_message_header {
() => {
// Module: crate::msgs::message
// Provides: {"read_opaque_message_header"}
// Dependencies: {}
pub (crate) fn read_opaque_message_header (r : & mut Reader < '_ > ,) -> Result < (ContentType , ProtocolVersion , u16) , MessageError > { let typ = ContentType :: read (r) . map_err (| _ | MessageError :: TooShortForHeader) ? ; if let ContentType :: Unknown (_) = typ { return Err (MessageError :: InvalidContentType) ; } let version = ProtocolVersion :: read (r) . map_err (| _ | MessageError :: TooShortForHeader) ? ; match & version { ProtocolVersion :: Unknown (v) if (v & 0xff00) != 0x0300 => { return Err (MessageError :: UnknownProtocolVersion) ; } _ => { } } ; let len = u16 :: read (r) . map_err (| _ | MessageError :: TooShortForHeader) ? ; if typ != ContentType :: ApplicationData && len == 0 { return Err (MessageError :: InvalidEmptyPayload) ; } if len >= MAX_PAYLOAD { return Err (MessageError :: MessageTooLarge) ; } Ok ((typ , version , len)) }
};
}
