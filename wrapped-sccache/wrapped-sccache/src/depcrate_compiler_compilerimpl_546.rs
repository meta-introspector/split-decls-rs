// Generated macro for impl_546 (impl)
macro_rules! Depcrate_compiler_compilerimpl_546 {
() => {
// Module: crate::compiler::compiler
// Provides: {"impl_546"}
// Dependencies: {}
# [async_trait] impl < T , I > CompileCommand < T > for CCompileCommand < I > where T : CommandCreatorSync , I : CompileCommandImpl , { fn get_executable (& self) -> PathBuf { self . cmd . get_executable () } fn get_arguments (& self) -> Vec < OsString > { self . cmd . get_arguments () } fn get_env_vars (& self) -> Vec < (OsString , OsString) > { self . cmd . get_env_vars () } fn get_cwd (& self) -> PathBuf { self . cmd . get_cwd () } async fn execute (& self , service : & server :: SccacheService < T > , creator : & T ,) -> Result < process :: Output > { self . cmd . execute (service , creator) . await } }
};
}
