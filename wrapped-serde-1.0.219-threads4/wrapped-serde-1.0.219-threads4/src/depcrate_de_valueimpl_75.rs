// Generated macro for impl_75 (impl)
macro_rules! Depcrate_de_valueimpl_75 {
() => {
// Module: crate::de::value
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'de , E > Debug for BorrowedStrDeserializer < 'de , E > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct ("BorrowedStrDeserializer") . field ("value" , & self . value) . finish () } }
};
}
