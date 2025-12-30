// Generated macro for impl_36 (impl)
macro_rules! Depcrate_intimpl_36 {
() => {
// Module: crate::int
// Provides: {"impl_36"}
// Dependencies: {}
impl < T , I > IntFrom < I > for T where I : TryInto < Self > , { fn int_from (int : I) -> Option < Self > { int . try_into () . ok () } }
};
}
