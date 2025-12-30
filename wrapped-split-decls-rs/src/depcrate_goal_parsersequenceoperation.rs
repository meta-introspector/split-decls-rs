// Generated macro for SequenceOperation (struct)
macro_rules! Depcrate_goal_parserSequenceOperation {
() => {
// Module: crate::goal_parser
// Provides: {"SequenceOperation"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct SequenceOperation { # [serde (rename = "type")] pub op_type : String , pub tasks : Vec < Task > , }
};
}
