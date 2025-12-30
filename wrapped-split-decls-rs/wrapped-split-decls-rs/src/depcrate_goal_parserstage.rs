// Generated macro for Stage (struct)
macro_rules! Depcrate_goal_parserStage {
() => {
// Module: crate::goal_parser
// Provides: {"Stage"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct Stage { pub name : String , pub description : String , # [serde (default)] pub processor_hint : Option < String > , # [serde (default)] pub inputs : Vec < Input > , # [serde (default)] pub outputs : Vec < Output > , pub operation : Operation , # [serde (default)] pub tasks : Vec < Task > , }
};
}
