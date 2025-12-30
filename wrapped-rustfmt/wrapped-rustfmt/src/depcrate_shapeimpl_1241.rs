// Generated macro for impl_1241 (impl)
macro_rules! Depcrate_shapeimpl_1241 {
() => {
// Module: crate::shape
// Provides: {"impl_1241"}
// Dependencies: {}
impl Add for Indent { type Output = Indent ; fn add (self , rhs : Indent) -> Indent { Indent { block_indent : self . block_indent + rhs . block_indent , alignment : self . alignment + rhs . alignment , } } }
};
}
