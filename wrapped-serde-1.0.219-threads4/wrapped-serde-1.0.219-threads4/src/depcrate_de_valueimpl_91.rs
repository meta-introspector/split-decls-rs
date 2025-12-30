// Generated macro for impl_91 (impl)
macro_rules! Depcrate_de_valueimpl_91 {
() => {
// Module: crate::de::value
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a , E > Debug for CowStrDeserializer < 'a , E > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct ("CowStrDeserializer") . field ("value" , & self . value) . finish () } }
};
}
