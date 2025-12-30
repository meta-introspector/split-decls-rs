// Generated macro for impl_246 (impl)
macro_rules! Depcrate_predicate_formsimpl_246 {
() => {
// Module: crate::predicate_forms
// Provides: {"impl_246"}
// Dependencies: {}
impl TryFrom < & WildString > for PredicationMask { type Error = String ; fn try_from (value : & WildString) -> Result < Self , Self :: Error > { value . wildcards () . find_map (| w | { if let Wildcard :: PredicateForms (mask) = w { Some (* mask) } else { None } }) . ok_or_else (| | "no predicate forms were specified in the name" . to_string ()) } }
};
}
