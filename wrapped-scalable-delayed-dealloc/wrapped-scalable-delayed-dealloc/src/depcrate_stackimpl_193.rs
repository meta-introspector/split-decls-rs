// Generated macro for impl_193 (impl)
macro_rules! Depcrate_stackimpl_193 {
() => {
// Module: crate::stack
// Provides: {"impl_193"}
// Dependencies: {}
impl < T : 'static > FromIterator < T > for Stack < T > { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let into_iter = iter . into_iter () ; let stack = Self :: default () ; into_iter . for_each (| v | { stack . push (v) ; }) ; stack } }
};
}
