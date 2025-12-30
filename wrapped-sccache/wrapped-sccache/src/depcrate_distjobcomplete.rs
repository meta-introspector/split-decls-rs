// Generated macro for JobComplete (struct)
macro_rules! Depcrate_distJobComplete {
() => {
// Module: crate::dist
// Provides: {"JobComplete"}
// Dependencies: {}
# [derive (Clone , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct JobComplete { pub output : ProcessOutput , pub outputs : Vec < (String , OutputData) > , }
};
}
