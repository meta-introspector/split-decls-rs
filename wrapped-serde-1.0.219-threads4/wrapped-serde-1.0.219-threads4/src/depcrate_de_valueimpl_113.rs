// Generated macro for impl_113 (impl)
macro_rules! Depcrate_de_valueimpl_113 {
() => {
// Module: crate::de::value
// Provides: {"impl_113"}
// Dependencies: {}
impl < I , E > Debug for SeqDeserializer < I , E > where I : Debug , { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct ("SeqDeserializer") . field ("iter" , & self . iter) . field ("count" , & self . count) . finish () } }
};
}
