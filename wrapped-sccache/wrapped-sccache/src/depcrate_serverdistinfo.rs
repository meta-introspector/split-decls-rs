// Generated macro for DistInfo (enum)
macro_rules! Depcrate_serverDistInfo {
() => {
// Module: crate::server
// Provides: {"DistInfo"}
// Dependencies: {}
# [doc = " Status of the dist client."] # [derive (Serialize , Deserialize , Clone , Debug)] pub enum DistInfo { Disabled (String) , # [cfg (feature = "dist-client")] NotConnected (Option < config :: HTTPUrl > , String) , # [cfg (feature = "dist-client")] SchedulerStatus (Option < config :: HTTPUrl > , dist :: SchedulerStatusResult) , }
};
}
