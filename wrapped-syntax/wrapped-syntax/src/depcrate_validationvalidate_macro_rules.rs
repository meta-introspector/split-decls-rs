// Generated macro for validate_macro_rules (function)
macro_rules! Depcrate_validationvalidate_macro_rules {
() => {
// Module: crate::validation
// Provides: {"validate_macro_rules"}
// Dependencies: {}
fn validate_macro_rules (mac : ast :: MacroRules , errors : & mut Vec < SyntaxError >) { if let Some (vis) = mac . visibility () { errors . push (SyntaxError :: new ("visibilities are not allowed on `macro_rules!` items" , vis . syntax () . text_range () ,)) ; } }
};
}
