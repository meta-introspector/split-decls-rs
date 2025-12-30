// Generated macro for Input (struct)
macro_rules! Depcrate_goal_parserInput {
() => {
// Module: crate::goal_parser
// Provides: {"Input"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct Input { pub name : String , # [serde (default)] pub from_stage : Option < String > , # [serde (default)] pub from_task : Option < String > , pub output_name : String , }
};
}
