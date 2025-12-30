// Generated macro for impl_968 (impl)
macro_rules! Depcrate_serdeimpl_968 {
() => {
// Module: crate::serde
// Provides: {"impl_968"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for OffsetDateTime { # [inline] fn deserialize < D : Deserializer < 'a > > (deserializer : D) -> Result < Self , D :: Error > { if cfg ! (feature = "serde-human-readable") && deserializer . is_human_readable () { deserializer . deserialize_any (Visitor :: < Self > (PhantomData)) } else { deserializer . deserialize_tuple (9 , Visitor :: < Self > (PhantomData)) } } }
};
}
