// Generated macro for SwitchOperation (struct)
macro_rules! Depcrate_goal_parserSwitchOperation {
() => {
// Module: crate::goal_parser
// Provides: {"SwitchOperation"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct SwitchOperation { # [serde (rename = "type")] pub op_type : String , pub match_on : String , pub cases : HashMap < String , Vec < Task > > , # [serde (default)] pub default : Option < Vec < Task > > , }
};
}
