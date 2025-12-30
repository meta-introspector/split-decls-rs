// Generated macro for impl_132 (impl)
macro_rules! Depcrate_formatters_junitimpl_132 {
() => {
// Module: crate::formatters::junit
// Provides: {"impl_132"}
// Dependencies: {}
impl < T : Write > JunitFormatter < T > { pub (crate) fn new (out : OutputLocation < T >) -> Self { Self { out , results : Vec :: new () } } fn write_message (& mut self , s : & str) -> io :: Result < () > { assert ! (! s . contains ('\n')) ; self . out . write_all (s . as_ref ()) } }
};
}
