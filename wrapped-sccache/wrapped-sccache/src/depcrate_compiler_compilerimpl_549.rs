// Generated macro for impl_549 (impl)
macro_rules! Depcrate_compiler_compilerimpl_549 {
() => {
// Module: crate::compiler::compiler
// Provides: {"impl_549"}
// Dependencies: {}
# [async_trait] impl CompileCommandImpl for SingleCompileCommand { fn get_executable (& self) -> PathBuf { self . executable . clone () } fn get_arguments (& self) -> Vec < OsString > { self . arguments . clone () } fn get_env_vars (& self) -> Vec < (OsString , OsString) > { self . env_vars . clone () } fn get_cwd (& self) -> PathBuf { self . cwd . clone () } async fn execute < T > (& self , _ : & server :: SccacheService < T > , creator : & T ,) -> Result < process :: Output > where T : CommandCreatorSync , { let SingleCompileCommand { executable , arguments , env_vars , cwd , } = self ; let mut cmd = creator . clone () . new_command_sync (executable) ; cmd . args (arguments) . env_clear () . envs (env_vars . to_vec ()) . current_dir (cwd) ; run_input_output (cmd , None) . await } }
};
}
