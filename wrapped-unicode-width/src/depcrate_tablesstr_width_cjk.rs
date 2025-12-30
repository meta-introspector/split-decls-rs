// Generated macro for str_width_cjk (function)
macro_rules! Depcrate_tablesstr_width_cjk {
() => {
// Module: crate::tables
// Provides: {"str_width_cjk"}
// Dependencies: {}
# [cfg (feature = "cjk")] # [inline] pub fn str_width_cjk (s : & str) -> usize { s . chars () . rfold ((0 , WidthInfo :: DEFAULT) , | (sum , next_info) , c | -> (usize , WidthInfo) { let (add , info) = width_in_str_cjk (c , next_info) ; (sum . wrapping_add_signed (isize :: from (add)) , info) } ,) . 0 }
};
}
