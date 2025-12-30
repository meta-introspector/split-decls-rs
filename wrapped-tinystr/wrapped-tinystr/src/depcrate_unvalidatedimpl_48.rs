// Generated macro for impl_48 (impl)
macro_rules! Depcrate_unvalidatedimpl_48 {
() => {
// Module: crate::unvalidated
// Provides: {"impl_48"}
// Dependencies: {}
impl < const N : usize > TinyAsciiStr < N > { # [inline] pub const fn to_unvalidated (self) -> UnvalidatedTinyAsciiStr < N > { UnvalidatedTinyAsciiStr (* self . all_bytes ()) } }
};
}
