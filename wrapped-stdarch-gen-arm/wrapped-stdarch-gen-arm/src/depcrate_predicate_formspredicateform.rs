// Generated macro for PredicateForm (enum)
macro_rules! Depcrate_predicate_formsPredicateForm {
() => {
// Module: crate::predicate_forms
// Provides: {"PredicateForm"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Serialize , Deserialize)] pub enum PredicateForm { # [doc = " Enables merging predicate form"] Merging , # [doc = " Enables \"don't care\" predicate form."] DontCare (DontCareMethod) , # [doc = " Enables zeroing predicate form. If LLVM zeroselection is performed, then"] # [doc = " set the `select` field to the variable that gets set. Otherwise set the"] # [doc = " `drop` field if the zeroinitializer replaces a predicate when merging."] Zeroing (ZeroingMethod) , }
};
}
