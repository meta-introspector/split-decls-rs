// Generated macro for impl_8 (impl)
macro_rules! Depcrate_integerimpl_8 {
() => {
// Module: crate::integer
// Provides: {"impl_8"}
// Dependencies: {}
impl < N > TomlInteger < N > where Self : crate :: WriteTomlValue , { # [doc = " Apply default formatting"] pub fn new (value : N) -> Self { Self { value , format : TomlIntegerFormat :: new () , } } }
};
}
