// Generated macro for Workflow (struct)
macro_rules! Depcrate_goal_parserWorkflow {
() => {
// Module: crate::goal_parser
// Provides: {"Workflow"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct Workflow { pub name : String , pub description : String , # [serde (default)] pub style_influences : Vec < String > , pub stages : Vec < Stage > , }
};
}
