// Generated macro for impl_71 (impl)
macro_rules! Depcrateimpl_71 {
() => {
// Module: crate
// Provides: {"impl_71"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < Z > serde :: Serialize for Zeroizing < Z > where Z : Zeroize + serde :: Serialize , { # [inline (always)] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { self . 0 . serialize (serializer) } }
};
}
