// Generated macro for impl_1242 (impl)
macro_rules! Depcrate_shapeimpl_1242 {
() => {
// Module: crate::shape
// Provides: {"impl_1242"}
// Dependencies: {}
impl Sub for Indent { type Output = Indent ; fn sub (self , rhs : Indent) -> Indent { Indent :: new (self . block_indent - rhs . block_indent , self . alignment - rhs . alignment ,) } }
};
}
