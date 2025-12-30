// Generated macro for impl_468 (impl)
macro_rules! Depcrate_compiler_ciccimpl_468 {
() => {
// Module: crate::compiler::cicc
// Provides: {"impl_468"}
// Dependencies: {}
# [async_trait] impl CCompilerImpl for Cicc { fn kind (& self) -> CCompilerKind { CCompilerKind :: Cicc } fn plusplus (& self) -> bool { true } fn version (& self) -> Option < String > { self . version . clone () } fn parse_arguments (& self , arguments : & [OsString] , cwd : & Path , _env_vars : & [(OsString , OsString)] ,) -> CompilerArguments < ParsedArguments > { parse_arguments (arguments , cwd , Language :: Ptx , & ARGS [..] , 3) } # [allow (clippy :: too_many_arguments)] async fn preprocess < T > (& self , _creator : & T , _executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , _env_vars : & [(OsString , OsString)] , _may_dist : bool , _rewrite_includes_only : bool , _preprocessor_cache_mode : bool ,) -> Result < process :: Output > where T : CommandCreatorSync , { preprocess (cwd , parsed_args) . await } fn generate_compile_commands < T > (& self , path_transformer : & mut dist :: PathTransformer , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] , _rewrite_includes_only : bool ,) -> Result < (Box < dyn CompileCommand < T > > , Option < dist :: CompileCommand > , Cacheable ,) > where T : CommandCreatorSync , { generate_compile_commands (path_transformer , executable , parsed_args , cwd , env_vars) . map (| (command , dist_command , cacheable) | { (CCompileCommand :: new (command) , dist_command , cacheable) } ,) } }
};
}
