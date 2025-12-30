// Generated macro for struct_lit_shape (function)
macro_rules! Depcrate_listsstruct_lit_shape {
() => {
// Module: crate::lists
// Provides: {"struct_lit_shape"}
// Dependencies: {}
pub (crate) fn struct_lit_shape (shape : Shape , context : & RewriteContext < '_ > , prefix_width : usize , suffix_width : usize , span : Span ,) -> Result < (Option < Shape > , Shape) , ExceedsMaxWidthError > { let v_shape = match context . config . indent_style () { IndentStyle :: Visual => shape . visual_indent (0) . shrink_left (prefix_width , span) ? . sub_width (suffix_width , span) ? , IndentStyle :: Block => { let shape = shape . block_indent (context . config . tab_spaces ()) ; Shape { width : context . budget (shape . indent . width ()) , .. shape } } } ; let h_shape = shape . width . checked_sub (prefix_width + suffix_width) . map (| w | { let shape_width = cmp :: min (w , context . config . struct_lit_width ()) ; Shape :: legacy (shape_width , shape . indent) }) ; Ok ((h_shape , v_shape)) }
};
}
