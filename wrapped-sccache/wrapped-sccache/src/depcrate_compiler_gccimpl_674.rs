// Generated macro for impl_674 (impl)
macro_rules! Depcrate_compiler_gccimpl_674 {
() => {
// Module: crate::compiler::gcc
// Provides: {"impl_674"}
// Dependencies: {}
impl < 'a > ExpandIncludeFile < 'a > { pub fn new (cwd : & 'a Path , args : & [OsString]) -> Self { ExpandIncludeFile { stack : args . iter () . rev () . map (| a | a . to_owned ()) . collect () , cwd , } } }
};
}
