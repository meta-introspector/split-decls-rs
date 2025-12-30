// Generated macro for impl_11 (impl)
macro_rules! Depcrate_rangeimpl_11 {
() => {
// Module: crate::range
// Provides: {"impl_11"}
// Dependencies: {}
impl IndexMut < TextRange > for str { # [inline] fn index_mut (& mut self , index : TextRange) -> & mut str { & mut self [Range :: < usize > :: from (index)] } }
};
}
