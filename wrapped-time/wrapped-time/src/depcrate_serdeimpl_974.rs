// Generated macro for impl_974 (impl)
macro_rules! Depcrate_serdeimpl_974 {
() => {
// Module: crate::serde
// Provides: {"impl_974"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for UtcDateTime { # [inline] fn deserialize < D : Deserializer < 'a > > (deserializer : D) -> Result < Self , D :: Error > { if cfg ! (feature = "serde-human-readable") && deserializer . is_human_readable () { deserializer . deserialize_any (Visitor :: < Self > (PhantomData)) } else { deserializer . deserialize_tuple (6 , Visitor :: < Self > (PhantomData)) } } }
};
}
