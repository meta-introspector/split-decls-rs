// Generated macro for impl_312 (impl)
macro_rules! Depcrate_deimpl_312 {
() => {
// Module: crate::de
// Provides: {"impl_312"}
// Dependencies: {}
impl < 'de , T > DeserializeSeed < 'de > for PhantomData < T > where T : Deserialize < 'de > , { type Value = T ; # [inline] fn deserialize < D > (self , deserializer : D) -> Result < T , D :: Error > where D : Deserializer < 'de > , { T :: deserialize (deserializer) } }
};
}
