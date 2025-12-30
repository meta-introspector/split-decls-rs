// Generated macro for impl_641 (impl)
macro_rules! Depcrate_compiler_diabimpl_641 {
() => {
// Module: crate::compiler::diab
// Provides: {"impl_641"}
// Dependencies: {}
impl < 'a > ExpandAtArgs < 'a > { pub fn new (cwd : & 'a Path , args : & [OsString]) -> Self { ExpandAtArgs { stack : args . iter () . rev () . map (| a | a . to_owned ()) . collect () , cwd , } } }
};
}
