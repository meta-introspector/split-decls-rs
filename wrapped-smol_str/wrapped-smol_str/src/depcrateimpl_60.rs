// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl < T > ToSmolStr for T where T : fmt :: Display + ? Sized , { fn to_smolstr (& self) -> SmolStr { format_smolstr ! ("{}" , self) } }
};
}
