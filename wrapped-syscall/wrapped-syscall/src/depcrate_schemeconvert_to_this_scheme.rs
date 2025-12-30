// Generated macro for convert_to_this_scheme (function)
macro_rules! Depcrate_schemeconvert_to_this_scheme {
() => {
// Module: crate::scheme
// Provides: {"convert_to_this_scheme"}
// Dependencies: {}
pub (crate) fn convert_to_this_scheme (r : Result < usize >) -> Result < OpenResult > { r . map (| number | OpenResult :: ThisScheme { number }) }
};
}
