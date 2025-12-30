// Generated macro for impl_67 (impl)
macro_rules! Depcrateimpl_67 {
() => {
// Module: crate
// Provides: {"impl_67"}
// Dependencies: {}
impl < T > FromIterator < T > for ThinVec < T > { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> ThinVec < T > { let mut vec = ThinVec :: new () ; vec . extend (iter) ; vec } }
};
}
