// Generated macro for shape_from_indent_style (function)
macro_rules! Depcrate_overflowshape_from_indent_style {
() => {
// Module: crate::overflow
// Provides: {"shape_from_indent_style"}
// Dependencies: {}
fn shape_from_indent_style (context : & RewriteContext < '_ > , shape : Shape , overhead : usize , offset : usize ,) -> Shape { let (shape , overhead) = if context . use_block_indent () { let shape = shape . block () . block_indent (context . config . tab_spaces ()) . with_max_width (context . config) ; (shape , 1) } else { (shape . visual_indent (offset) , overhead) } ; Shape { width : shape . width . saturating_sub (overhead) , .. shape } }
};
}
