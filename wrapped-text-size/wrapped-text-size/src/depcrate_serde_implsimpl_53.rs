// Generated macro for impl_53 (impl)
macro_rules! Depcrate_serde_implsimpl_53 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_53"}
// Dependencies: {}
impl Serialize for TextRange { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { (self . start () , self . end ()) . serialize (serializer) } }
};
}
