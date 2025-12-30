// Generated macro for impl_174 (impl)
macro_rules! Depcrateimpl_174 {
() => {
// Module: crate
// Provides: {"impl_174"}
// Dependencies: {}
impl < HCX > HashStable < HCX > for LintId { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . lint_name_raw () . hash_stable (hcx , hasher) ; } }
};
}
