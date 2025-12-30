// Generated macro for impl_169 (impl)
macro_rules! Depcrate_serial_numberimpl_169 {
() => {
// Module: crate::serial_number
// Provides: {"impl_169"}
// Dependencies: {}
impl < P : Profile > EncodeValue for SerialNumber < P > { fn value_len (& self) -> Result < Length > { self . inner . value_len () } fn encode_value (& self , writer : & mut impl Writer) -> Result < () > { self . inner . encode_value (writer) } }
};
}
