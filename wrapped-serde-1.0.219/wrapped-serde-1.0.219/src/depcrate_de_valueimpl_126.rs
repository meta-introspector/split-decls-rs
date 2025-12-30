// Generated macro for impl_126 (impl)
macro_rules! Depcrate_de_valueimpl_126 {
() => {
// Module: crate::de::value
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'de , I , E > IntoDeserializer < 'de , E > for MapDeserializer < 'de , I , E > where I : Iterator , I :: Item : private :: Pair , First < I :: Item > : IntoDeserializer < 'de , E > , Second < I :: Item > : IntoDeserializer < 'de , E > , E : de :: Error , { type Deserializer = Self ; fn into_deserializer (self) -> Self { self } }
};
}
