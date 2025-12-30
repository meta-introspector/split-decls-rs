// Generated macro for CompileCommand (trait)
macro_rules! Depcrate_compiler_compilerCompileCommand {
() => {
// Module: crate::compiler::compiler
// Provides: {"CompileCommand"}
// Dependencies: {}
# [async_trait] pub trait CompileCommand < T > : Send + Sync + 'static where T : CommandCreatorSync , { async fn execute (& self , service : & server :: SccacheService < T > , creator : & T ,) -> Result < process :: Output > ; fn get_executable (& self) -> PathBuf ; fn get_arguments (& self) -> Vec < OsString > ; fn get_env_vars (& self) -> Vec < (OsString , OsString) > ; fn get_cwd (& self) -> PathBuf ; }
};
}
