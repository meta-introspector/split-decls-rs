// Generated macro for impl_213 (impl)
macro_rules! Depcrate_subject_nameimpl_213 {
() => {
// Module: crate::subject_name
// Provides: {"impl_213"}
// Dependencies: {}
impl < 'a > Iterator for NameIterator < 'a > { type Item = Result < GeneralName < 'a > , Error > ; fn next (& mut self) -> Option < Self :: Item > { let subject_alt_name = self . subject_alt_name . as_mut () ? ; if subject_alt_name . at_end () { self . subject_alt_name = None ; return None ; } let err = match GeneralName :: from_der (subject_alt_name) { Ok (name) => return Some (Ok (name)) , Err (err) => err , } ; self . subject_alt_name = None ; Some (Err (err)) } }
};
}
