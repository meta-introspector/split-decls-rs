// Generated macro for impl_965 (impl)
macro_rules! Depcrate_serdeimpl_965 {
() => {
// Module: crate::serde
// Provides: {"impl_965"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for Duration { # [inline] fn deserialize < D : Deserializer < 'a > > (deserializer : D) -> Result < Self , D :: Error > { if cfg ! (feature = "serde-human-readable") && deserializer . is_human_readable () { deserializer . deserialize_any (Visitor :: < Self > (PhantomData)) } else { deserializer . deserialize_tuple (2 , Visitor :: < Self > (PhantomData)) } } }
};
}
