// Generated macro for Task (struct)
macro_rules! Depcrate_goal_parserTask {
() => {
// Module: crate::goal_parser
// Provides: {"Task"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct Task { pub name : String , pub operation : Operation , # [serde (default)] pub inputs : Vec < Input > , # [serde (default)] pub outputs : Vec < Output > , }
};
}
