// Generated macro for impl_37 (impl)
macro_rules! Depcrate_sizeimpl_37 {
() => {
// Module: crate::size
// Provides: {"impl_37"}
// Dependencies: {}
impl < A > iter :: Sum < A > for TextSize where TextSize : Add < A , Output = TextSize > , { # [inline] fn sum < I : Iterator < Item = A > > (iter : I) -> TextSize { iter . fold (0 . into () , Add :: add) } }
};
}
