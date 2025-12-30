// Generated macro for convert_to_this_scheme_block (function)
macro_rules! Depcrate_schemeconvert_to_this_scheme_block {
() => {
// Module: crate::scheme
// Provides: {"convert_to_this_scheme_block"}
// Dependencies: {}
pub (crate) fn convert_to_this_scheme_block (r : Result < Option < usize > >) -> Result < Option < OpenResult > > { r . map (| o | o . map (| number | OpenResult :: ThisScheme { number })) }
};
}
