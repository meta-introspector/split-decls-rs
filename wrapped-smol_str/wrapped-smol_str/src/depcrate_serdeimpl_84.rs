// Generated macro for impl_84 (impl)
macro_rules! Depcrate_serdeimpl_84 {
() => {
// Module: crate::serde
// Provides: {"impl_84"}
// Dependencies: {}
impl serde :: Serialize for SmolStr { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { self . as_str () . serialize (serializer) } }
};
}
