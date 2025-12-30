// Generated macro for impl_212 (impl)
macro_rules! Depcrate_subject_nameimpl_212 {
() => {
// Module: crate::subject_name
// Provides: {"impl_212"}
// Dependencies: {}
impl < 'a > NameIterator < 'a > { pub (crate) fn new (subject_alt_name : Option < untrusted :: Input < 'a > >) -> Self { Self { subject_alt_name : subject_alt_name . map (untrusted :: Reader :: new) , } } }
};
}
