// Generated macro for impl_70 (impl)
macro_rules! Depcrate_private_keyimpl_70 {
() => {
// Module: crate::private_key
// Provides: {"impl_70"}
// Dependencies: {}
impl EncodeValue for EcPrivateKey < '_ > { fn value_len (& self) -> der :: Result < Length > { VERSION . encoded_len () ? + OctetStringRef :: new (self . private_key) ? . encoded_len () ? + self . context_specific_parameters () . encoded_len () ? + self . context_specific_public_key () ? . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { VERSION . encode (writer) ? ; OctetStringRef :: new (self . private_key) ? . encode (writer) ? ; self . context_specific_parameters () . encode (writer) ? ; self . context_specific_public_key () ? . encode (writer) ? ; Ok (()) } }
};
}
