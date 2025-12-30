// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
# [cfg (feature = "serde")] impl Serialize for Tai64 { fn serialize < S : ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { self . to_bytes () . serialize (serializer) } }
};
}
