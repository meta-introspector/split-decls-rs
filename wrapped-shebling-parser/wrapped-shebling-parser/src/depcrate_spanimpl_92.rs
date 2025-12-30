// Generated macro for impl_92 (impl)
macro_rules! Depcrate_spanimpl_92 {
() => {
// Module: crate::span
// Provides: {"impl_92"}
// Dependencies: {}
impl nom :: Slice < RangeFrom < usize > > for ParseSpan < '_ > { fn slice (& self , range : RangeFrom < usize >) -> Self { let new_fragment = & self . fragment . slice (range) ; let new_offset = self . offset + self . fragment . offset (new_fragment) ; ParseSpan { fragment : new_fragment , offset : new_offset , diags : self . diags , } } }
};
}
