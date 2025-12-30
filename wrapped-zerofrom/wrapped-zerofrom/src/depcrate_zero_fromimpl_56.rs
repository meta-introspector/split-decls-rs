// Generated macro for impl_56 (impl)
macro_rules! Depcrate_zero_fromimpl_56 {
() => {
// Module: crate::zero_from
// Provides: {"impl_56"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'zf , B : ToOwned + ? Sized > ZeroFrom < 'zf , Cow < '_ , B > > for Cow < 'zf , B > { # [inline] fn zero_from (other : & 'zf Cow < '_ , B >) -> Self { Cow :: Borrowed (other) } }
};
}
