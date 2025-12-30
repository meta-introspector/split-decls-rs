// Generated macro for FunctionCallOperation (struct)
macro_rules! Depcrate_goal_parserFunctionCallOperation {
() => {
// Module: crate::goal_parser
// Provides: {"FunctionCallOperation"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct FunctionCallOperation { # [serde (rename = "type")] pub op_type : String , pub function : String , # [serde (default)] pub args : Vec < String > , # [serde (default)] pub recursive : Option < bool > , # [serde (default)] pub output_base : Option < String > , }
};
}
