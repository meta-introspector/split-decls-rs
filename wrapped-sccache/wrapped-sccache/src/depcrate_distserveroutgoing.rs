// Generated macro for ServerOutgoing (trait)
macro_rules! Depcrate_distServerOutgoing {
() => {
// Module: crate::dist
// Provides: {"ServerOutgoing"}
// Dependencies: {}
# [cfg (feature = "dist-server")] pub trait ServerOutgoing { fn do_update_job_state (& self , job_id : JobId , state : JobState) -> Result < UpdateJobStateResult > ; }
};
}
