// Generated macro for impl_190 (impl)
macro_rules! Depcrate_filter_utilsimpl_190 {
() => {
// Module: crate::filter_utils
// Provides: {"impl_190"}
// Dependencies: {}
impl GetValue for OrderedF64 { fn get_value (val : & Value) -> Result < Self > { let n = val . as_f64 () . ok_or_else (| | Error :: msg (format ! ("expected number got {}" , val))) ? ; Ok (OrderedF64 :: new (n)) } }
};
}
