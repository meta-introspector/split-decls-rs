// Generated macro for impl_86 (impl)
macro_rules! Depcrate_deimpl_86 {
() => {
// Module: crate::de
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'de , const N : usize > Deserialize < 'de > for & 'de [u8 ; N] { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let arr : & ByteArray < N > = serde :: Deserialize :: deserialize (deserializer) ? ; Ok (arr) } }
};
}
