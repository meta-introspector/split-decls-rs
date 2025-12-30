// Generated macro for impl_543 (impl)
macro_rules! Depcrate_datetime_legacyimpl_543 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_543"}
// Dependencies: {}
impl < 'a > From < & weekdays :: Symbols < 'a > > for LinearNames < 'a > { fn from (other : & weekdays :: Symbols < 'a >) -> Self { let vec : alloc :: vec :: Vec < & str > = other . 0 . iter () . map (| x | & * * x) . collect () ; LinearNames { names : (& vec) . into () , } } }
};
}
