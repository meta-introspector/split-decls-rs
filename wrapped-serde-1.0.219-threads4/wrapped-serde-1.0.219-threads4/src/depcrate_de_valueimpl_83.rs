// Generated macro for impl_83 (impl)
macro_rules! Depcrate_de_valueimpl_83 {
() => {
// Module: crate::de::value
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < E > Debug for StringDeserializer < E > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct ("StringDeserializer") . field ("value" , & self . value) . finish () } }
};
}
