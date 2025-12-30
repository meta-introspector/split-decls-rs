// Generated macro for impl_51 (impl)
macro_rules! Depcrate_serde_implsimpl_51 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_51"}
// Dependencies: {}
impl Serialize for TextSize { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . raw . serialize (serializer) } }
};
}
