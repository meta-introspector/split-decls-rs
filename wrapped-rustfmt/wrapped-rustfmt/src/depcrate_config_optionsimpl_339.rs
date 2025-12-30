// Generated macro for impl_339 (impl)
macro_rules! Depcrate_config_optionsimpl_339 {
() => {
// Module: crate::config::options
// Provides: {"impl_339"}
// Dependencies: {}
impl WidthHeuristics { pub fn null () -> WidthHeuristics { WidthHeuristics { fn_call_width : usize :: MAX , attr_fn_like_width : usize :: MAX , struct_lit_width : 0 , struct_variant_width : 0 , array_width : usize :: MAX , chain_width : usize :: MAX , single_line_if_else_max_width : 0 , single_line_let_else_max_width : 0 , } } pub fn set (max_width : usize) -> WidthHeuristics { WidthHeuristics { fn_call_width : max_width , attr_fn_like_width : max_width , struct_lit_width : max_width , struct_variant_width : max_width , array_width : max_width , chain_width : max_width , single_line_if_else_max_width : max_width , single_line_let_else_max_width : max_width , } } pub fn scaled (max_width : usize) -> WidthHeuristics { const DEFAULT_MAX_WIDTH : usize = 100 ; let max_width_ratio = if max_width > DEFAULT_MAX_WIDTH { let ratio = max_width as f32 / DEFAULT_MAX_WIDTH as f32 ; (ratio * 10.0) . round () / 10.0 } else { 1.0 } ; WidthHeuristics { fn_call_width : (60.0 * max_width_ratio) . round () as usize , attr_fn_like_width : (70.0 * max_width_ratio) . round () as usize , struct_lit_width : (18.0 * max_width_ratio) . round () as usize , struct_variant_width : (35.0 * max_width_ratio) . round () as usize , array_width : (60.0 * max_width_ratio) . round () as usize , chain_width : (60.0 * max_width_ratio) . round () as usize , single_line_if_else_max_width : (50.0 * max_width_ratio) . round () as usize , single_line_let_else_max_width : (50.0 * max_width_ratio) . round () as usize , } } }
};
}
