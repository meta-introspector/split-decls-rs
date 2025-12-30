// Generated macro for impl_93 (impl)
macro_rules! Depcrate_spanimpl_93 {
() => {
// Module: crate::span
// Provides: {"impl_93"}
// Dependencies: {}
impl nom :: Slice < RangeTo < usize > > for ParseSpan < '_ > { fn slice (& self , range : RangeTo < usize >) -> Self { let new_fragment = & self . fragment . slice (range) ; let new_offset = self . offset + self . fragment . offset (new_fragment) ; ParseSpan { fragment : new_fragment , offset : new_offset , diags : self . diags , } } }
};
}
