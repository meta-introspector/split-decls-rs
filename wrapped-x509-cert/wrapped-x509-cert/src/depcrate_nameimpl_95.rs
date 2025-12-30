// Generated macro for impl_95 (impl)
macro_rules! Depcrate_nameimpl_95 {
() => {
// Module: crate::name
// Provides: {"impl_95"}
// Dependencies: {}
impl EncodeValue for Name { fn encode_value (& self , encoder : & mut impl Writer) -> der :: Result < () > { self . 0 . encode_value (encoder) } fn value_len (& self) -> der :: Result < Length > { self . 0 . value_len () } }
};
}
