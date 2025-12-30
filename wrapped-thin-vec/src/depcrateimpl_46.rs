// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl < T > Extend < T > for ThinVec < T > { # [inline] fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = T > , { let iter = iter . into_iter () ; let hint = iter . size_hint () . 0 ; if hint > 0 { self . reserve (hint) ; } for x in iter { self . push (x) ; } } }
};
}
