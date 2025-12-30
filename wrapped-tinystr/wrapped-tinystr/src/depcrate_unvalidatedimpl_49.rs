// Generated macro for impl_49 (impl)
macro_rules! Depcrate_unvalidatedimpl_49 {
() => {
// Module: crate::unvalidated
// Provides: {"impl_49"}
// Dependencies: {}
impl < const N : usize > From < TinyAsciiStr < N > > for UnvalidatedTinyAsciiStr < N > { fn from (other : TinyAsciiStr < N >) -> Self { other . to_unvalidated () } }
};
}
