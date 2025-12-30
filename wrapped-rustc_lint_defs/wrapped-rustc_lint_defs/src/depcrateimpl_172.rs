// Generated macro for impl_172 (impl)
macro_rules! Depcrateimpl_172 {
() => {
// Module: crate
// Provides: {"impl_172"}
// Dependencies: {}
impl std :: hash :: Hash for LintId { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { let ptr = self . lint as * const Lint ; ptr . hash (state) ; } }
};
}
