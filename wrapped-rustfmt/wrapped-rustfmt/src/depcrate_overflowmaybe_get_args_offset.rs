// Generated macro for maybe_get_args_offset (function)
macro_rules! Depcrate_overflowmaybe_get_args_offset {
() => {
// Module: crate::overflow
// Provides: {"maybe_get_args_offset"}
// Dependencies: {}
# [doc = " In case special-case style is required, returns an offset from which we start horizontal layout."] pub (crate) fn maybe_get_args_offset (callee_str : & str , args : & [OverflowableItem < '_ >] , config : & Config ,) -> Option < (bool , usize) > { if let Some (& (_ , num_args_before)) = args . get (0) ? . special_cases (config) . find (| & & (s , _) | s == callee_str) { let all_simple = args . len () > num_args_before && is_every_expr_simple (& args [0 .. num_args_before]) && is_every_expr_simple (& args [num_args_before + 1 ..]) ; Some ((all_simple , num_args_before)) } else { None } }
};
}
