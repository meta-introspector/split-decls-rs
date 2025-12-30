// Generated macro for GoalConfig (struct)
macro_rules! Depcrate_goal_parserGoalConfig {
() => {
// Module: crate::goal_parser
// Provides: {"GoalConfig"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct GoalConfig { # [serde (rename = "original-goal")] pub original_goal : Option < String > , pub workflow : Workflow , }
};
}
