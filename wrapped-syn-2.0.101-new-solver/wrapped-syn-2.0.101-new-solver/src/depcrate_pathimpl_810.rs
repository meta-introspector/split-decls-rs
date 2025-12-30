// Generated macro for impl_810 (impl)
macro_rules! Depcrate_pathimpl_810 {
() => {
// Module: crate::path
// Provides: {"impl_810"}
// Dependencies: {}
impl < T > From < T > for Path where T : Into < PathSegment > , { fn from (segment : T) -> Self { let mut path = Path { leading_colon : None , segments : Punctuated :: new () , } ; path . segments . push_value (segment . into ()) ; path } }
};
}
