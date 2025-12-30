// Generated macro for struct_field_prefix_max_min_width (function)
macro_rules! Depcrate_verticalstruct_field_prefix_max_min_width {
() => {
// Module: crate::vertical
// Provides: {"struct_field_prefix_max_min_width"}
// Dependencies: {}
fn struct_field_prefix_max_min_width < T : AlignedItem > (context : & RewriteContext < '_ > , fields : & [T] , shape : Shape ,) -> (usize , usize) { fields . iter () . map (| field | { field . rewrite_prefix (context , shape) . map (| field_str | trimmed_last_line_width (& field_str)) }) . fold_ok ((0 , :: std :: usize :: MAX) , | (max_len , min_len) , len | { (cmp :: max (max_len , len) , cmp :: min (min_len , len)) }) . unwrap_or ((0 , 0)) }
};
}
