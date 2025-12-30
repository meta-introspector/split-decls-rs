// Generated macro for impl_43 (impl)
macro_rules! Depcrate_pkeimpl_43 {
() => {
// Module: crate::pke
// Provides: {"impl_43"}
// Dependencies: {}
impl EncodeValue for Cipher < '_ > { fn value_len (& self) -> der :: Result < Length > { UintRef :: new (& self . x . to_be_bytes ()) ? . encoded_len () ? + UintRef :: new (& self . y . to_be_bytes ()) ? . encoded_len () ? + OctetStringRef :: new (self . digest) ? . encoded_len () ? + OctetStringRef :: new (self . cipher) ? . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { UintRef :: new (& self . x . to_be_bytes ()) ? . encode (writer) ? ; UintRef :: new (& self . y . to_be_bytes ()) ? . encode (writer) ? ; OctetStringRef :: new (self . digest) ? . encode (writer) ? ; OctetStringRef :: new (self . cipher) ? . encode (writer) ? ; Ok (()) } }
};
}
