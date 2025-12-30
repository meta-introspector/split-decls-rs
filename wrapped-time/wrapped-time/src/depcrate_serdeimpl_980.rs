// Generated macro for impl_980 (impl)
macro_rules! Depcrate_serdeimpl_980 {
() => {
// Module: crate::serde
// Provides: {"impl_980"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for UtcOffset { # [inline] fn deserialize < D : Deserializer < 'a > > (deserializer : D) -> Result < Self , D :: Error > { if cfg ! (feature = "serde-human-readable") && deserializer . is_human_readable () { deserializer . deserialize_any (Visitor :: < Self > (PhantomData)) } else { deserializer . deserialize_tuple (3 , Visitor :: < Self > (PhantomData)) } } }
};
}
