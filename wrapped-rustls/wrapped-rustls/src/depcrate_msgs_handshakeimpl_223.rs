// Generated macro for impl_223 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_223 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_223"}
// Dependencies: {}
impl Codec < '_ > for SessionId { fn encode (& self , bytes : & mut Vec < u8 >) { debug_assert ! (self . len <= 32) ; bytes . push (self . len as u8) ; bytes . extend_from_slice (self . as_ref ()) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let len = u8 :: read (r) ? as usize ; if len > 32 { return Err (InvalidMessage :: TrailingData ("SessionID")) ; } let Some (bytes) = r . take (len) else { return Err (InvalidMessage :: MissingData ("SessionID")) ; } ; let mut out = [0u8 ; 32] ; out [.. len] . clone_from_slice (& bytes [.. len]) ; Ok (Self { data : out , len }) } }
};
}
