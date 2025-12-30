// Generated macro for validate_placeholder (function)
macro_rules! Depcrate_filter_redactionsvalidate_placeholder {
() => {
// Module: crate::filter::redactions
// Provides: {"validate_placeholder"}
// Dependencies: {}
fn validate_placeholder (placeholder : & 'static str) -> crate :: assert :: Result < & 'static str > { if ! placeholder . starts_with ('[') || ! placeholder . ends_with (']') { return Err (format ! ("Key `{placeholder}` is not enclosed in []") . into ()) ; } if placeholder [1 .. (placeholder . len () - 1)] . find (| c : char | ! c . is_ascii_uppercase () && c != '_') . is_some () { return Err (format ! ("Key `{placeholder}` can only be A-Z but ") . into ()) ; } Ok (placeholder) }
};
}
