// Generated macro for impl_192 (impl)
macro_rules! Depcrate_filter_utilsimpl_192 {
() => {
// Module: crate::filter_utils
// Provides: {"impl_192"}
// Dependencies: {}
impl GetValue for bool { fn get_value (val : & Value) -> Result < Self > { val . as_bool () . ok_or_else (| | Error :: msg (format ! ("expected bool got {}" , val))) } }
};
}
