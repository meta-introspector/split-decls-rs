// Generated macro for impl_24 (impl)
macro_rules! Depcrate_pointimpl_24 {
() => {
// Module: crate::point
// Provides: {"impl_24"}
// Dependencies: {}
impl < Size : ModulusSize > Ord for EncodedPoint < Size > where Size : ModulusSize , { fn cmp (& self , other : & Self) -> Ordering { self . as_bytes () . cmp (other . as_bytes ()) } }
};
}
