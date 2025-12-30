// Generated macro for impl_937 (impl)
macro_rules! Depcrate_compiler_tasking_vximpl_937 {
() => {
// Module: crate::compiler::tasking_vx
// Provides: {"impl_937"}
// Dependencies: {}
# [async_trait] impl CCompilerImpl for TaskingVX { fn kind (& self) -> CCompilerKind { CCompilerKind :: TaskingVX } fn plusplus (& self) -> bool { false } fn version (& self) -> Option < String > { None } fn parse_arguments (& self , arguments : & [OsString] , cwd : & Path , _env_vars : & [(OsString , OsString)] ,) -> CompilerArguments < ParsedArguments > { parse_arguments (arguments , cwd , & ARGS [..]) } async fn preprocess < T > (& self , creator : & T , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , may_dist : bool , rewrite_includes_only : bool , _preprocessor_cache_mode : bool ,) -> Result < process :: Output > where T : CommandCreatorSync , { preprocess (creator , executable , parsed_args , cwd , env_vars , may_dist , rewrite_includes_only ,) . await } fn generate_compile_commands < T > (& self , path_transformer : & mut dist :: PathTransformer , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , _rewrite_includes_only : bool ,) -> Result < (Box < dyn CompileCommand < T > > , Option < dist :: CompileCommand > , Cacheable ,) > where T : CommandCreatorSync , { generate_compile_commands (path_transformer , executable , parsed_args , cwd , env_vars) . map (| (command , dist_command , cacheable) | { (CCompileCommand :: new (command) , dist_command , cacheable) } ,) } }
};
}
