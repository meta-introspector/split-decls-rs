// Generated macro for impl_12 (impl)
macro_rules! Depcrate_rangeimpl_12 {
() => {
// Module: crate::range
// Provides: {"impl_12"}
// Dependencies: {}
impl IndexMut < TextRange > for String { # [inline] fn index_mut (& mut self , index : TextRange) -> & mut str { & mut self [Range :: < usize > :: from (index)] } }
};
}
