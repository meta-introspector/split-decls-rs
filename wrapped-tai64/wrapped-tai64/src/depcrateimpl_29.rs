// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
# [cfg (feature = "serde")] impl Serialize for Tai64N { fn serialize < S : ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { self . to_bytes () . serialize (serializer) } }
};
}
