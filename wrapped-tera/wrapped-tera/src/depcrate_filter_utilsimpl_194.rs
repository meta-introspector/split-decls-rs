// Generated macro for impl_194 (impl)
macro_rules! Depcrate_filter_utilsimpl_194 {
() => {
// Module: crate::filter_utils
// Provides: {"impl_194"}
// Dependencies: {}
impl GetValue for ArrayLen { fn get_value (val : & Value) -> Result < Self > { let arr = val . as_array () . ok_or_else (| | Error :: msg (format ! ("expected array got {}" , val))) ? ; Ok (ArrayLen (arr . len ())) } }
};
}
