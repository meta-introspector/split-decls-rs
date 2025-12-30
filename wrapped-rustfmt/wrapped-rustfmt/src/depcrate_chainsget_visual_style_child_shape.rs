// Generated macro for get_visual_style_child_shape (function)
macro_rules! Depcrate_chainsget_visual_style_child_shape {
() => {
// Module: crate::chains
// Provides: {"get_visual_style_child_shape"}
// Dependencies: {}
fn get_visual_style_child_shape (context : & RewriteContext < '_ > , shape : Shape , offset : usize , parent_overflowing : bool , span : Span ,) -> Result < Shape , ExceedsMaxWidthError > { if ! parent_overflowing { shape . with_max_width (context . config) . offset_left (offset , span) . map (| s | s . visual_indent (0)) } else { Ok (shape . visual_indent (offset)) } }
};
}
