// Generated macro for impl_197 (impl)
macro_rules! Depcrateimpl_197 {
() => {
// Module: crate
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'a > form_urlencoded :: Target for UrlQuery < 'a > { fn as_mut_string (& mut self) -> & mut String { & mut self . url . as_mut () . unwrap () . serialization } fn finish (mut self) -> & 'a mut Url { let url = self . url . take () . unwrap () ; url . restore_already_parsed_fragment (self . fragment . take ()) ; url } type Finished = & 'a mut Url ; }
};
}
