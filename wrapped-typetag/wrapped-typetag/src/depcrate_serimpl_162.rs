// Generated macro for impl_162 (impl)
macro_rules! Depcrate_serimpl_162 {
() => {
// Module: crate::ser
// Provides: {"impl_162"}
// Dependencies: {}
impl < 'a , T > Serialize for Wrap < 'a , T > where T : ? Sized + erased_serde :: Serialize + 'a , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { erased_serde :: serialize (self . 0 , serializer) } }
};
}
