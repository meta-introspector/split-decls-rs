// Generated macro for impl_21 (impl)
macro_rules! Depcrate_pointimpl_21 {
() => {
// Module: crate::point
// Provides: {"impl_21"}
// Dependencies: {}
impl < Size > PartialEq for EncodedPoint < Size > where Size : ModulusSize , { fn eq (& self , other : & Self) -> bool { self . as_bytes () == other . as_bytes () } }
};
}
