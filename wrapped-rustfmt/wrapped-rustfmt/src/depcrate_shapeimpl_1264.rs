// Generated macro for impl_1264 (impl)
macro_rules! Depcrate_shapeimpl_1264 {
() => {
// Module: crate::shape
// Provides: {"impl_1264"}
// Dependencies: {}
impl Sub for Indent { type Output = Indent ; fn sub (self , rhs : Indent) -> Indent { Indent :: new (self . block_indent - rhs . block_indent , self . alignment - rhs . alignment ,) } }
};
}
