// Generated macro for SchedulerStatusResult (struct)
macro_rules! Depcrate_distSchedulerStatusResult {
() => {
// Module: crate::dist
// Provides: {"SchedulerStatusResult"}
// Dependencies: {}
# [derive (Clone , Debug , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct SchedulerStatusResult { pub num_servers : usize , pub num_cpus : usize , pub in_progress : usize , }
};
}
