// Generated macro for impl_193 (impl)
macro_rules! Depcrate_filter_utilsimpl_193 {
() => {
// Module: crate::filter_utils
// Provides: {"impl_193"}
// Dependencies: {}
impl GetValue for String { fn get_value (val : & Value) -> Result < Self > { let str : Result < & str > = val . as_str () . ok_or_else (| | Error :: msg (format ! ("expected string got {}" , val))) ; Ok (str ? . to_owned ()) } }
};
}
