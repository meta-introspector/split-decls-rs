// Generated macro for impl_104 (impl)
macro_rules! Depcrate_de_valueimpl_104 {
() => {
// Module: crate::de::value
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'de , E > Debug for BorrowedBytesDeserializer < 'de , E > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct ("BorrowedBytesDeserializer") . field ("value" , & self . value) . finish () } }
};
}
