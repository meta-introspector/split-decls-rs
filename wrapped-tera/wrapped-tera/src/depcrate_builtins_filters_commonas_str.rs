// Generated macro for as_str (function)
macro_rules! Depcrate_builtins_filters_commonas_str {
() => {
// Module: crate::builtins::filters::common
// Provides: {"as_str"}
// Dependencies: {}
pub fn as_str (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let value = render_to_string (| | format ! ("as_str for value of kind {}" , value) , | w | value . render (w)) ? ; to_value (value) . map_err (Error :: json) }
};
}
