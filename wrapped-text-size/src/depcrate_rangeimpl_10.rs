// Generated macro for impl_10 (impl)
macro_rules! Depcrate_rangeimpl_10 {
() => {
// Module: crate::range
// Provides: {"impl_10"}
// Dependencies: {}
impl Index < TextRange > for String { type Output = str ; # [inline] fn index (& self , index : TextRange) -> & str { & self [Range :: < usize > :: from (index)] } }
};
}
