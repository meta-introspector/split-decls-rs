// Generated macro for impl_356 (impl)
macro_rules! Depcrate_ser_implsimpl_356 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_356"}
// Dependencies: {}
impl < 'a > Serialize for fmt :: Arguments < 'a > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_str (self) } }
};
}
