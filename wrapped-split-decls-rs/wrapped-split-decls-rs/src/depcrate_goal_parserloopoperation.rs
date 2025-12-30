// Generated macro for LoopOperation (struct)
macro_rules! Depcrate_goal_parserLoopOperation {
() => {
// Module: crate::goal_parser
// Provides: {"LoopOperation"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct LoopOperation { # [serde (rename = "type")] pub op_type : String , pub over : String , pub loop_var : String , pub tasks : Vec < Task > , }
};
}
