// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < T : Sanitize > Sanitize for [T] { fn sanitize (& self) -> Result < () , SanitizeError > { for x in self . iter () { x . sanitize () ? ; } Ok (()) } }
};
}
