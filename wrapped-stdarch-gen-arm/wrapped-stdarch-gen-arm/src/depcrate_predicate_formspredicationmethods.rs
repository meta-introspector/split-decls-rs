// Generated macro for PredicationMethods (struct)
macro_rules! Depcrate_predicate_formsPredicationMethods {
() => {
// Module: crate::predicate_forms
// Provides: {"PredicationMethods"}
// Dependencies: {}
# [derive (Debug , Clone , Default , PartialEq , Eq , Deserialize , Serialize)] pub struct PredicationMethods { # [doc = " Zeroing method, if the zeroing predicate form is used"] # [serde (default)] pub zeroing_method : Option < ZeroingMethod > , # [doc = " Don't care method, if the don't care predicate form is used"] # [serde (default)] pub dont_care_method : DontCareMethod , }
};
}
