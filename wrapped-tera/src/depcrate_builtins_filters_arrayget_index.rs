// Generated macro for get_index (function)
macro_rules! Depcrate_builtins_filters_arrayget_index {
() => {
// Module: crate::builtins::filters::array
// Provides: {"get_index"}
// Dependencies: {}
# [inline] fn get_index (i : f64 , array : & [Value]) -> usize { if i >= 0.0 { i as usize } else { (array . len () as f64 + i) as usize } }
};
}
