// Generated macro for impl_191 (impl)
macro_rules! Depcrate_filter_utilsimpl_191 {
() => {
// Module: crate::filter_utils
// Provides: {"impl_191"}
// Dependencies: {}
impl GetValue for i64 { fn get_value (val : & Value) -> Result < Self > { val . as_i64 () . ok_or_else (| | Error :: msg (format ! ("expected number got {}" , val))) } }
};
}
