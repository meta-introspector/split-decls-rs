// Generated macro for impl_244 (impl)
macro_rules! Depcrate_predicate_formsimpl_244 {
() => {
// Module: crate::predicate_forms
// Provides: {"impl_244"}
// Dependencies: {}
impl FromStr for PredicationMask { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut result = Self :: default () ; for kind in s . bytes () { match kind { b'm' => result . m = true , b'x' => result . x = true , b'z' => result . z = true , _ => { return Err (format ! ("unknown predicate form modifier: {}" , char :: from (kind))) ; } } } if result . m || result . x || result . z { Ok (result) } else { Err ("invalid predication mask" . to_string ()) } } }
};
}
