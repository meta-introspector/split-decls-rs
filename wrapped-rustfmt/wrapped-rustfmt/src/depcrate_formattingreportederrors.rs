// Generated macro for ReportedErrors (struct)
macro_rules! Depcrate_formattingReportedErrors {
() => {
// Module: crate::formatting
// Provides: {"ReportedErrors"}
// Dependencies: {}
# [derive (Default , Debug , PartialEq)] pub (crate) struct ReportedErrors { pub (crate) has_operational_errors : bool , pub (crate) has_parsing_errors : bool , pub (crate) has_formatting_errors : bool , pub (crate) has_macro_format_failure : bool , pub (crate) has_check_errors : bool , # [doc = " Formatted code differs from existing code (--check only)."] pub (crate) has_diff : bool , # [doc = " Formatted code missed something, like lost comments or extra trailing space"] pub (crate) has_unformatted_code_errors : bool , }
};
}
