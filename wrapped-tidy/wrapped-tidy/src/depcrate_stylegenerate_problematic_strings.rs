// Generated macro for generate_problematic_strings (function)
macro_rules! Depcrate_stylegenerate_problematic_strings {
() => {
// Module: crate::style
// Provides: {"generate_problematic_strings"}
// Dependencies: {}
fn generate_problematic_strings (consts : & [u32] , letter_digit : & FxHashMap < char , char > ,) -> Vec < String > { generate_problems (consts , letter_digit) . flat_map (| v | vec ! [v . to_string () , format ! ("{:X}" , v)]) . collect () }
};
}
