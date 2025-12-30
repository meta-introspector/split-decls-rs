// Generated macro for SchedulerOutgoing (trait)
macro_rules! Depcrate_distSchedulerOutgoing {
() => {
// Module: crate::dist
// Provides: {"SchedulerOutgoing"}
// Dependencies: {}
# [cfg (feature = "dist-server")] pub trait SchedulerOutgoing { fn do_assign_job (& self , server_id : ServerId , job_id : JobId , tc : Toolchain , auth : String ,) -> Result < AssignJobResult > ; }
};
}
