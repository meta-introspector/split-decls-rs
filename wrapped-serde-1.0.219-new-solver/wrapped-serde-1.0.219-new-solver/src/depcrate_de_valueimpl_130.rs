// Generated macro for impl_130 (impl)
macro_rules! Depcrate_de_valueimpl_130 {
() => {
// Module: crate::de::value
// Provides: {"impl_130"}
// Dependencies: {}
impl < 'de , I , E > Debug for MapDeserializer < 'de , I , E > where I : Iterator + Debug , I :: Item : private :: Pair , Second < I :: Item > : Debug , { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_struct ("MapDeserializer") . field ("iter" , & self . iter) . field ("value" , & self . value) . field ("count" , & self . count) . finish () } }
};
}
