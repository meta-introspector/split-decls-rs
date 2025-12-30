// Generated macro for generics_shape_from_config (function)
macro_rules! Depcrate_itemsgenerics_shape_from_config {
() => {
// Module: crate::items
// Provides: {"generics_shape_from_config"}
// Dependencies: {}
fn generics_shape_from_config (config : & Config , shape : Shape , offset : usize , span : Span ,) -> Result < Shape , ExceedsMaxWidthError > { match config . indent_style () { IndentStyle :: Visual => shape . visual_indent (1 + offset) . sub_width (offset + 2 , span) , IndentStyle :: Block => { shape . block () . block_indent (config . tab_spaces ()) . with_max_width (config) . sub_width (1 , span) } } }
};
}
