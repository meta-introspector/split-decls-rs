// Generated macro for impl_633 (impl)
macro_rules! Depcrate_compiler_diabimpl_633 {
() => {
// Module: crate::compiler::diab
// Provides: {"impl_633"}
// Dependencies: {}
# [async_trait] impl CCompilerImpl for Diab { fn kind (& self) -> CCompilerKind { CCompilerKind :: Diab } fn plusplus (& self) -> bool { false } fn version (& self) -> Option < String > { self . version . clone () } fn parse_arguments (& self , arguments : & [OsString] , cwd : & Path , _env_vars : & [(OsString , OsString)] ,) -> CompilerArguments < ParsedArguments > { parse_arguments (arguments , cwd , & ARGS [..]) } # [allow (clippy :: too_many_arguments)] async fn preprocess < T > (& self , creator : & T , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , may_dist : bool , _rewrite_includes_only : bool , _preprocessor_cache_mode : bool ,) -> Result < process :: Output > where T : CommandCreatorSync , { preprocess (creator , executable , parsed_args , cwd , env_vars , may_dist) . await } fn generate_compile_commands < T > (& self , path_transformer : & mut dist :: PathTransformer , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , _rewrite_includes_only : bool ,) -> Result < (Box < dyn CompileCommand < T > > , Option < dist :: CompileCommand > , Cacheable ,) > where T : CommandCreatorSync , { generate_compile_commands (path_transformer , executable , parsed_args , cwd , env_vars) . map (| (command , dist_command , cacheable) | { (CCompileCommand :: new (command) , dist_command , cacheable) } ,) } }
};
}
