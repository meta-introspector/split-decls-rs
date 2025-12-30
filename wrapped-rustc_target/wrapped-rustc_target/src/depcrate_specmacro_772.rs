// Generated macro for macro_772 (macro)
macro_rules! Depcrate_specmacro_772 {
() => {
// Module: crate::spec
// Provides: {"macro_772"}
// Dependencies: {}
crate :: target_spec_enum ! { # [doc = " Controls use of stack canaries."] pub enum StackProtector { # [doc = " Disable stack canary generation."] None = "none" , # [doc = " On LLVM, mark all generated LLVM functions with the `ssp` attribute (see"] # [doc = " llvm/docs/LangRef.rst). This triggers stack canary generation in"] # [doc = " functions which contain an array of a byte-sized type with more than"] # [doc = " eight elements."] Basic = "basic" , # [doc = " On LLVM, mark all generated LLVM functions with the `sspstrong`"] # [doc = " attribute (see llvm/docs/LangRef.rst). This triggers stack canary"] # [doc = " generation in functions which either contain an array, or which take"] # [doc = " the address of a local variable."] Strong = "strong" , # [doc = " Generate stack canaries in all functions."] All = "all" , } parse_error_type = "stack protector" ; }
};
}
