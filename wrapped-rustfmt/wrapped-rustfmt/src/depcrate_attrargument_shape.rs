// Generated macro for argument_shape (function)
macro_rules! Depcrate_attrargument_shape {
() => {
// Module: crate::attr
// Provides: {"argument_shape"}
// Dependencies: {}
fn argument_shape (left : usize , right : usize , combine : bool , shape : Shape , context : & RewriteContext < '_ > ,) -> Option < Shape > { let shape = match context . config . indent_style () { IndentStyle :: Block => { if combine { shape . offset_left_opt (left) ? } else { shape . block_indent (context . config . tab_spaces ()) . with_max_width (context . config) } } IndentStyle :: Visual => shape . visual_indent (0) . shrink_left_opt (left) ? . sub_width_opt (right) ? , } ; Some (shape) }
};
}
