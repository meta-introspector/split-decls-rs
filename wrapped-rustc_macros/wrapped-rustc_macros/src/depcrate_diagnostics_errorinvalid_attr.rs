// Generated macro for invalid_attr (function)
macro_rules! Depcrate_diagnostics_errorinvalid_attr {
() => {
// Module: crate::diagnostics::error
// Provides: {"invalid_attr"}
// Dependencies: {}
# [doc = " Returns an error diagnostic for an invalid attribute."] pub (crate) fn invalid_attr (attr : & Attribute) -> Diagnostic { let span = attr . span () . unwrap () ; let path = path_to_string (attr . path ()) ; match attr . meta { Meta :: Path (_) => span_err (span , format ! ("`#[{path}]` is not a valid attribute")) , Meta :: NameValue (_) => span_err (span , format ! ("`#[{path} = ...]` is not a valid attribute")) , Meta :: List (_) => span_err (span , format ! ("`#[{path}(...)]` is not a valid attribute")) , } }
};
}
