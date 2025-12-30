// Generated macro for impl_246 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_246 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_246"}
// Dependencies: {}
impl Codec < '_ > for SingleProtocolName { fn encode (& self , bytes : & mut Vec < u8 >) { let body = LengthPrefixedBuffer :: new (Self :: SIZE_LEN , bytes) ; self . 0 . encode (body . buf) ; } fn read (reader : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let len = Self :: SIZE_LEN . read (reader) ? ; let mut sub = reader . sub (len) ? ; let item = ProtocolName :: read (& mut sub) ? ; if sub . any_left () { Err (InvalidMessage :: TrailingData ("SingleProtocolName")) } else { Ok (Self (item)) } } }
};
}
