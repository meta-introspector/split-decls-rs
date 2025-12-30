// Generated macro for value_by_pointer (function)
macro_rules! Depcrate_renderer_stack_framevalue_by_pointer {
() => {
// Module: crate::renderer::stack_frame
// Provides: {"value_by_pointer"}
// Dependencies: {}
# [doc = " Gets a value within a value by pointer, keeping lifetime"] # [inline] pub fn value_by_pointer < 'a > (pointer : & str , val : & Val < 'a >) -> Option < Val < 'a > > { match * val { Cow :: Borrowed (r) => dotted_pointer (r , pointer) . map (Cow :: Borrowed) , Cow :: Owned (ref r) => dotted_pointer (r , pointer) . map (| found | Cow :: Owned (found . clone ())) , } }
};
}
