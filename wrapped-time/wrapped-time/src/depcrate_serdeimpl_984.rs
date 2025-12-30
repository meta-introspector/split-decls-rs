// Generated macro for impl_984 (impl)
macro_rules! Depcrate_serdeimpl_984 {
() => {
// Module: crate::serde
// Provides: {"impl_984"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for Month { # [inline] fn deserialize < D : Deserializer < 'a > > (deserializer : D) -> Result < Self , D :: Error > { if cfg ! (feature = "serde-human-readable") && deserializer . is_human_readable () { deserializer . deserialize_any (Visitor :: < Self > (PhantomData)) } else { deserializer . deserialize_u8 (Visitor :: < Self > (PhantomData)) } } }
};
}
