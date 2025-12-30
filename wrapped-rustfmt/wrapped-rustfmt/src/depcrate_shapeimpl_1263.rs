// Generated macro for impl_1263 (impl)
macro_rules! Depcrate_shapeimpl_1263 {
() => {
// Module: crate::shape
// Provides: {"impl_1263"}
// Dependencies: {}
impl Add for Indent { type Output = Indent ; fn add (self , rhs : Indent) -> Indent { Indent { block_indent : self . block_indent + rhs . block_indent , alignment : self . alignment + rhs . alignment , } } }
};
}
