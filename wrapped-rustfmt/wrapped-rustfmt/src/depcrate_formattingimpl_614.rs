// Generated macro for impl_614 (impl)
macro_rules! Depcrate_formattingimpl_614 {
() => {
// Module: crate::formatting
// Provides: {"impl_614"}
// Dependencies: {}
impl ReportedErrors { # [doc = " Combine two summaries together."] pub (crate) fn add (& mut self , other : & ReportedErrors) { self . has_operational_errors |= other . has_operational_errors ; self . has_parsing_errors |= other . has_parsing_errors ; self . has_formatting_errors |= other . has_formatting_errors ; self . has_macro_format_failure |= other . has_macro_format_failure ; self . has_check_errors |= other . has_check_errors ; self . has_diff |= other . has_diff ; self . has_unformatted_code_errors |= other . has_unformatted_code_errors ; } }
};
}
