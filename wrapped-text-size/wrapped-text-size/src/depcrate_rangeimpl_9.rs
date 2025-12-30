// Generated macro for impl_9 (impl)
macro_rules! Depcrate_rangeimpl_9 {
() => {
// Module: crate::range
// Provides: {"impl_9"}
// Dependencies: {}
impl Index < TextRange > for str { type Output = str ; # [inline] fn index (& self , index : TextRange) -> & str { & self [Range :: < usize > :: from (index)] } }
};
}
