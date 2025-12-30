// Generated macro for HashResult (struct)
macro_rules! Depcrate_compiler_compilerHashResult {
() => {
// Module: crate::compiler::compiler
// Provides: {"HashResult"}
// Dependencies: {}
# [doc = " Result of generating a hash from a compiler command."] pub struct HashResult < T > where T : CommandCreatorSync , { # [doc = " The hash key of the inputs."] pub key : String , # [doc = " An object to use for the actual compilation, if necessary."] pub compilation : Box < dyn Compilation < T > + 'static > , # [doc = " A weak key that may be used to identify the toolchain"] pub weak_toolchain_key : String , }
};
}
