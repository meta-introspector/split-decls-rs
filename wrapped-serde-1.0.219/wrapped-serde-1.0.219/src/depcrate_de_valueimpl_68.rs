// Generated macro for impl_68 (impl)
macro_rules! Depcrate_de_valueimpl_68 {
() => {
// Module: crate::de::value
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a , E > Debug for StrDeserializer < 'a , E > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct ("StrDeserializer") . field ("value" , & self . value) . finish () } }
};
}
