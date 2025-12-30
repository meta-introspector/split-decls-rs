// Generated macro for impl_217 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_217 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_217"}
// Dependencies: {}
impl Codec < '_ > for Random { fn encode (& self , bytes : & mut Vec < u8 >) { bytes . extend_from_slice (& self . 0) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let Some (bytes) = r . take (32) else { return Err (InvalidMessage :: MissingData ("Random")) ; } ; let mut opaque = [0 ; 32] ; opaque . clone_from_slice (bytes) ; Ok (Self (opaque)) } }
};
}
