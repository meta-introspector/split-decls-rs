// Generated macro for impl_23 (impl)
macro_rules! Depcrate_pointimpl_23 {
() => {
// Module: crate::point
// Provides: {"impl_23"}
// Dependencies: {}
impl < Size : ModulusSize > PartialOrd for EncodedPoint < Size > where Size : ModulusSize , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
