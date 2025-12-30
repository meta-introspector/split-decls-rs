// Generated macro for impl_710 (impl)
macro_rules! Depcrate_compiler_msvcimpl_710 {
() => {
// Module: crate::compiler::msvc
// Provides: {"impl_710"}
// Dependencies: {}
impl < 'a > ExpandIncludeFile < 'a > { pub fn new (cwd : & 'a Path , args : & [OsString]) -> Self { ExpandIncludeFile { args : args . iter () . rev () . map (| a | a . to_owned ()) . collect () , stack : Vec :: new () , cwd , } } }
};
}
