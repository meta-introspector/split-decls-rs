// Generated macro for impl_695 (impl)
macro_rules! Depcrate_compiler_msvcimpl_695 {
() => {
// Module: crate::compiler::msvc
// Provides: {"impl_695"}
// Dependencies: {}
# [async_trait] impl CCompilerImpl for Msvc { fn kind (& self) -> CCompilerKind { CCompilerKind :: Msvc } fn plusplus (& self) -> bool { false } fn version (& self) -> Option < String > { self . version . clone () } fn parse_arguments (& self , arguments : & [OsString] , cwd : & Path , _env_vars : & [(OsString , OsString)] ,) -> CompilerArguments < ParsedArguments > { parse_arguments (arguments , cwd , self . is_clang) } # [allow (clippy :: too_many_arguments)] async fn preprocess < T > (& self , creator : & T , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , may_dist : bool , rewrite_includes_only : bool , _preprocessor_cache_mode : bool ,) -> Result < process :: Output > where T : CommandCreatorSync , { preprocess (creator , executable , parsed_args , cwd , env_vars , may_dist , & self . includes_prefix , rewrite_includes_only , self . is_clang ,) . await } fn generate_compile_commands < T > (& self , path_transformer : & mut dist :: PathTransformer , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , _rewrite_includes_only : bool ,) -> Result < (Box < dyn CompileCommand < T > > , Option < dist :: CompileCommand > , Cacheable ,) > where T : CommandCreatorSync , { generate_compile_commands (path_transformer , executable , parsed_args , cwd , env_vars) . map (| (command , dist_command , cacheable) | { (CCompileCommand :: new (command) , dist_command , cacheable) } ,) } }
};
}
