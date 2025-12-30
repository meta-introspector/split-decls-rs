// Generated macro for impl_85 (impl)
macro_rules! Depcrate_deimpl_85 {
() => {
// Module: crate::de
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'de , const N : usize > Deserialize < 'de > for [u8 ; N] { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let arr : ByteArray < N > = serde :: Deserialize :: deserialize (deserializer) ? ; Ok (* arr) } }
};
}
