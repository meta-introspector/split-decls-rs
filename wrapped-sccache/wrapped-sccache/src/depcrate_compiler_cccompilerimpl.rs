// Generated macro for CCompilerImpl (trait)
macro_rules! Depcrate_compiler_cCCompilerImpl {
() => {
// Module: crate::compiler::c
// Provides: {"CCompilerImpl"}
// Dependencies: {}
# [doc = " An interface to a specific C compiler."] # [async_trait] pub trait CCompilerImpl : Clone + fmt :: Debug + Send + Sync + 'static { # [doc = " Return the kind of compiler."] fn kind (& self) -> CCompilerKind ; # [doc = " Return true iff this is g++ or clang++."] fn plusplus (& self) -> bool ; # [doc = " Return the compiler version reported by the compiler executable."] fn version (& self) -> Option < String > ; # [doc = " Determine whether `arguments` are supported by this compiler."] fn parse_arguments (& self , arguments : & [OsString] , cwd : & Path , env_vars : & [(OsString , OsString)] ,) -> CompilerArguments < ParsedArguments > ; # [doc = " Run the C preprocessor with the specified set of arguments."] # [allow (clippy :: too_many_arguments)] async fn preprocess < T > (& self , creator : & T , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , may_dist : bool , rewrite_includes_only : bool , preprocessor_cache_mode : bool ,) -> Result < process :: Output > where T : CommandCreatorSync ; # [doc = " Generate a command that can be used to invoke the C compiler to perform"] # [doc = " the compilation."] fn generate_compile_commands < T > (& self , path_transformer : & mut dist :: PathTransformer , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , rewrite_includes_only : bool ,) -> Result < (Box < dyn CompileCommand < T > > , Option < dist :: CompileCommand > , Cacheable ,) > where T : CommandCreatorSync ; }
};
}
