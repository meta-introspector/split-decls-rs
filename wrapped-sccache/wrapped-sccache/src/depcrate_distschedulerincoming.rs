// Generated macro for SchedulerIncoming (trait)
macro_rules! Depcrate_distSchedulerIncoming {
() => {
// Module: crate::dist
// Provides: {"SchedulerIncoming"}
// Dependencies: {}
# [cfg (feature = "dist-server")] pub trait SchedulerIncoming : Send + Sync { fn handle_alloc_job (& self , requester : & dyn SchedulerOutgoing , tc : Toolchain ,) -> ExtResult < AllocJobResult , Error > ; fn handle_heartbeat_server (& self , server_id : ServerId , server_nonce : ServerNonce , num_cpus : usize , job_authorizer : Box < dyn JobAuthorizer > ,) -> ExtResult < HeartbeatServerResult , Error > ; fn handle_update_job_state (& self , job_id : JobId , server_id : ServerId , job_state : JobState ,) -> ExtResult < UpdateJobStateResult , Error > ; fn handle_status (& self) -> ExtResult < SchedulerStatusResult , Error > ; }
};
}
