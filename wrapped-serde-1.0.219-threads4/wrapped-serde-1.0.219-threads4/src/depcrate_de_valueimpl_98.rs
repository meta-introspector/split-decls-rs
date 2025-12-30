// Generated macro for impl_98 (impl)
macro_rules! Depcrate_de_valueimpl_98 {
() => {
// Module: crate::de::value
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a , E > Debug for BytesDeserializer < 'a , E > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct ("BytesDeserializer") . field ("value" , & self . value) . finish () } }
};
}
