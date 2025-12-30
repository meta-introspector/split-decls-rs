// Generated macro for impl_216 (impl)
macro_rules! Depcrate_fieldimpl_216 {
() => {
// Module: crate::field
// Provides: {"impl_216"}
// Dependencies: {}
impl Iterator for Iter { type Item = Field ; # [inline] fn next (& mut self) -> Option < Field > { let i = self . idxs . next () ? ; Some (Field { i , fields : FieldSet { names : self . fields . names , callsite : self . fields . callsite () , } , }) } }
};
}
