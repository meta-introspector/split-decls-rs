// Generated macro for str_width (function)
macro_rules! Depcrate_tablesstr_width {
() => {
// Module: crate::tables
// Provides: {"str_width"}
// Dependencies: {}
# [inline] pub fn str_width (s : & str) -> usize { s . chars () . rfold ((0 , WidthInfo :: DEFAULT) , | (sum , next_info) , c | -> (usize , WidthInfo) { let (add , info) = width_in_str (c , next_info) ; (sum . wrapping_add_signed (isize :: from (add)) , info) } ,) . 0 }
};
}
