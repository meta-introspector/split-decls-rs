// Generated macro for serialize_hex (function)
macro_rules! Depcrate_commonserialize_hex {
() => {
// Module: crate::common
// Provides: {"serialize_hex"}
// Dependencies: {}
pub (crate) fn serialize_hex < S , T , const UPPERCASE : bool > (value : & T , serializer : S ,) -> Result < S :: Ok , S :: Error > where S : Serializer , T : AsRef < [u8] > , { # [cfg (feature = "alloc")] if UPPERCASE { base16ct :: upper :: encode_string (value . as_ref ()) . serialize (serializer) } else { base16ct :: lower :: encode_string (value . as_ref ()) . serialize (serializer) } # [cfg (not (feature = "alloc"))] { let _ = value ; let _ = serializer ; Err (S :: Error :: custom ("serializer is human readable, which requires the `alloc` crate feature" ,)) } }
};
}
