// Generated macro for impl_714 (impl)
macro_rules! Depcrate_compiler_msvcimpl_714 {
() => {
// Module: crate::compiler::msvc
// Provides: {"impl_714"}
// Dependencies: {}
impl < 'a , T > From < & 'a T > for SplitMsvcResponseFileArgs < 'a > where T : AsRef < str > + 'static , { fn from (file_content : & 'a T) -> Self { Self { file_content : file_content . as_ref () , } } }
};
}
