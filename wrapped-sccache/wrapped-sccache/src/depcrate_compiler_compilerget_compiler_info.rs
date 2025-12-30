// Generated macro for get_compiler_info (function)
macro_rules! Depcrate_compiler_compilerget_compiler_info {
() => {
// Module: crate::compiler::compiler
// Provides: {"get_compiler_info"}
// Dependencies: {}
# [doc = " If `executable` is a known compiler, return a `Box<Compiler>` containing information about it."] pub async fn get_compiler_info < T > (creator : T , executable : & Path , cwd : & Path , args : & [OsString] , env : & [(OsString , OsString)] , pool : & tokio :: runtime :: Handle , dist_archive : Option < PathBuf > ,) -> Result < (Box < dyn Compiler < T > > , Option < Box < dyn CompilerProxy < T > > >) > where T : CommandCreatorSync , { let pool = pool . clone () ; detect_compiler (creator , executable , cwd , args , env , & pool , dist_archive) . await }
};
}
