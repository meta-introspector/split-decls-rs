// Generated macro for ServerIncoming (trait)
macro_rules! Depcrate_distServerIncoming {
() => {
// Module: crate::dist
// Provides: {"ServerIncoming"}
// Dependencies: {}
# [cfg (feature = "dist-server")] pub trait ServerIncoming : Send + Sync { fn handle_assign_job (& self , job_id : JobId , tc : Toolchain) -> ExtResult < AssignJobResult , Error > ; fn handle_submit_toolchain (& self , requester : & dyn ServerOutgoing , job_id : JobId , tc_rdr : ToolchainReader < '_ > ,) -> ExtResult < SubmitToolchainResult , Error > ; fn handle_run_job (& self , requester : & dyn ServerOutgoing , job_id : JobId , command : CompileCommand , outputs : Vec < String > , inputs_rdr : InputsReader < '_ > ,) -> ExtResult < RunJobResult , Error > ; }
};
}
