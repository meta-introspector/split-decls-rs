// Generated macro for impl_963 (impl)
macro_rules! Depcrate_serdeimpl_963 {
() => {
// Module: crate::serde
// Provides: {"impl_963"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for Date { # [inline] fn deserialize < D : Deserializer < 'a > > (deserializer : D) -> Result < Self , D :: Error > { if cfg ! (feature = "serde-human-readable") && deserializer . is_human_readable () { deserializer . deserialize_any (Visitor :: < Self > (PhantomData)) } else { deserializer . deserialize_tuple (2 , Visitor :: < Self > (PhantomData)) } } }
};
}
