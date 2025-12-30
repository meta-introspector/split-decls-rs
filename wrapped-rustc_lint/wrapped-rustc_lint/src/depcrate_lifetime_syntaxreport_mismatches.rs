// Generated macro for report_mismatches (function)
macro_rules! Depcrate_lifetime_syntaxreport_mismatches {
() => {
// Module: crate::lifetime_syntax
// Provides: {"report_mismatches"}
// Dependencies: {}
# [instrument (skip_all)] fn report_mismatches < 'tcx > (cx : & LateContext < 'tcx > , inputs : & LifetimeInfoMap < 'tcx > , outputs : & LifetimeInfoMap < 'tcx > ,) { for (resolved_lifetime , output_info) in outputs { if let Some (input_info) = inputs . get (resolved_lifetime) { if ! lifetimes_use_matched_syntax (input_info , output_info) { emit_mismatch_diagnostic (cx , input_info , output_info) ; } } } }
};
}
