// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , Z > serde :: Deserialize < 'de > for Zeroizing < Z > where Z : Zeroize + serde :: Deserialize < 'de > , { # [inline (always)] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { Ok (Self (Z :: deserialize (deserializer) ?)) } }
};
}
