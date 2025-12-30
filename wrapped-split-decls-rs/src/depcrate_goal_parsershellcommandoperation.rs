// Generated macro for ShellCommandOperation (struct)
macro_rules! Depcrate_goal_parserShellCommandOperation {
() => {
// Module: crate::goal_parser
// Provides: {"ShellCommandOperation"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct ShellCommandOperation { # [serde (rename = "type")] pub op_type : String , pub command : String , # [serde (default)] pub working_dir : Option < String > , # [serde (default = "default_true")] pub capture_output : bool , # [serde (default)] pub error_on_failure : bool , }
};
}
