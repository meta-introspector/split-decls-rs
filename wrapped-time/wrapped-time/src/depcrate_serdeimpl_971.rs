// Generated macro for impl_971 (impl)
macro_rules! Depcrate_serdeimpl_971 {
() => {
// Module: crate::serde
// Provides: {"impl_971"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for PrimitiveDateTime { # [inline] fn deserialize < D : Deserializer < 'a > > (deserializer : D) -> Result < Self , D :: Error > { if cfg ! (feature = "serde-human-readable") && deserializer . is_human_readable () { deserializer . deserialize_any (Visitor :: < Self > (PhantomData)) } else { deserializer . deserialize_tuple (6 , Visitor :: < Self > (PhantomData)) } } }
};
}
