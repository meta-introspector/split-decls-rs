// Generated macro for impl_982 (impl)
macro_rules! Depcrate_serdeimpl_982 {
() => {
// Module: crate::serde
// Provides: {"impl_982"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for Weekday { # [inline] fn deserialize < D : Deserializer < 'a > > (deserializer : D) -> Result < Self , D :: Error > { if cfg ! (feature = "serde-human-readable") && deserializer . is_human_readable () { deserializer . deserialize_any (Visitor :: < Self > (PhantomData)) } else { deserializer . deserialize_u8 (Visitor :: < Self > (PhantomData)) } } }
};
}
