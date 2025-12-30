// Generated macro for Client (trait)
macro_rules! Depcrate_distClient {
() => {
// Module: crate::dist
// Provides: {"Client"}
// Dependencies: {}
# [async_trait] pub trait Client : Send + Sync { async fn do_alloc_job (& self , tc : Toolchain) -> Result < AllocJobResult > ; async fn do_get_status (& self) -> Result < SchedulerStatusResult > ; async fn do_submit_toolchain (& self , job_alloc : JobAlloc , tc : Toolchain ,) -> Result < SubmitToolchainResult > ; async fn do_run_job (& self , job_alloc : JobAlloc , command : CompileCommand , outputs : Vec < String > , inputs_packager : Box < dyn pkg :: InputsPackager > ,) -> Result < (RunJobResult , PathTransformer) > ; async fn put_toolchain (& self , compiler_path : PathBuf , weak_key : String , toolchain_packager : Box < dyn pkg :: ToolchainPackager > ,) -> Result < (Toolchain , Option < (String , PathBuf) >) > ; fn rewrite_includes_only (& self) -> bool ; fn get_custom_toolchain (& self , exe : & Path) -> Option < PathBuf > ; }
};
}
