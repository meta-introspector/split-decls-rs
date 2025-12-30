// Generated macro for impl_977 (impl)
macro_rules! Depcrate_serdeimpl_977 {
() => {
// Module: crate::serde
// Provides: {"impl_977"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for Time { # [inline] fn deserialize < D : Deserializer < 'a > > (deserializer : D) -> Result < Self , D :: Error > { if cfg ! (feature = "serde-human-readable") && deserializer . is_human_readable () { deserializer . deserialize_any (Visitor :: < Self > (PhantomData)) } else { deserializer . deserialize_tuple (4 , Visitor :: < Self > (PhantomData)) } } }
};
}
