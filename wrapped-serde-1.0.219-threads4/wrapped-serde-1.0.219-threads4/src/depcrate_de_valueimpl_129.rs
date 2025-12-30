// Generated macro for impl_129 (impl)
macro_rules! Depcrate_de_valueimpl_129 {
() => {
// Module: crate::de::value
// Provides: {"impl_129"}
// Dependencies: {}
impl < 'de , I , E > Clone for MapDeserializer < 'de , I , E > where I : Iterator + Clone , I :: Item : private :: Pair , Second < I :: Item > : Clone , { fn clone (& self) -> Self { MapDeserializer { iter : self . iter . clone () , value : self . value . clone () , count : self . count , lifetime : self . lifetime , error : self . error , } } }
};
}
