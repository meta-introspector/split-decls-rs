// Generated macro for impl_325 (impl)
macro_rules! Depcrate_verify_certimpl_325 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_325"}
// Dependencies: {}
impl Budget { # [inline] pub (crate) fn consume_signature (& mut self) -> Result < () , Error > { self . signatures = self . signatures . checked_sub (1) . ok_or (Error :: MaximumSignatureChecksExceeded) ? ; Ok (()) } # [inline] fn consume_build_chain_call (& mut self) -> Result < () , Error > { self . build_chain_calls = self . build_chain_calls . checked_sub (1) . ok_or (Error :: MaximumPathBuildCallsExceeded) ? ; Ok (()) } # [inline] pub (crate) fn consume_name_constraint_comparison (& mut self) -> Result < () , Error > { self . name_constraint_comparisons = self . name_constraint_comparisons . checked_sub (1) . ok_or (Error :: MaximumNameConstraintComparisonsExceeded) ? ; Ok (()) } }
};
}
