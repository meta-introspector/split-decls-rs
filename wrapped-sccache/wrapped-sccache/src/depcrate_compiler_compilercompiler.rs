// Generated macro for Compiler (trait)
macro_rules! Depcrate_compiler_compilerCompiler {
() => {
// Module: crate::compiler::compiler
// Provides: {"Compiler"}
// Dependencies: {}
# [doc = " An interface to a compiler for argument parsing."] pub trait Compiler < T > : Send + Sync + 'static where T : CommandCreatorSync , { # [doc = " Return the kind of compiler."] fn kind (& self) -> CompilerKind ; # [doc = " Retrieve a packager"] # [cfg (feature = "dist-client")] fn get_toolchain_packager (& self) -> Box < dyn pkg :: ToolchainPackager > ; # [doc = " Determine whether `arguments` are supported by this compiler."] fn parse_arguments (& self , arguments : & [OsString] , cwd : & Path , env_vars : & [(OsString , OsString)] ,) -> CompilerArguments < Box < dyn CompilerHasher < T > + 'static > > ; fn box_clone (& self) -> Box < dyn Compiler < T > > ; }
};
}
