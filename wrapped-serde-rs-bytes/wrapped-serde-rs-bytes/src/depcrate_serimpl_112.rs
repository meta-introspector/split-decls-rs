// Generated macro for impl_112 (impl)
macro_rules! Depcrate_serimpl_112 {
() => {
// Module: crate::ser
// Provides: {"impl_112"}
// Dependencies: {}
impl < T > Serialize for & T where T : ? Sized + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { (* * self) . serialize (serializer) } }
};
}
