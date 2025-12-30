// Generated macro for CompileCommandImpl (trait)
macro_rules! Depcrate_compiler_compilerCompileCommandImpl {
() => {
// Module: crate::compiler::compiler
// Provides: {"CompileCommandImpl"}
// Dependencies: {}
# [async_trait] pub trait CompileCommandImpl : Send + Sync + 'static { fn get_executable (& self) -> PathBuf ; fn get_arguments (& self) -> Vec < OsString > ; fn get_env_vars (& self) -> Vec < (OsString , OsString) > ; fn get_cwd (& self) -> PathBuf ; async fn execute < T > (& self , service : & server :: SccacheService < T > , creator : & T ,) -> Result < process :: Output > where T : CommandCreatorSync ; }
};
}
