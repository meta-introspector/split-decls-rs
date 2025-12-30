// Generated macro for get_prefix_space_width (function)
macro_rules! Depcrate_utilsget_prefix_space_width {
() => {
// Module: crate::utils
// Provides: {"get_prefix_space_width"}
// Dependencies: {}
fn get_prefix_space_width (config : & Config , s : & str) -> usize { let mut width = 0 ; for c in s . chars () { match c { ' ' => width += 1 , '\t' => width += config . tab_spaces () , _ => return width , } } width }
};
}
