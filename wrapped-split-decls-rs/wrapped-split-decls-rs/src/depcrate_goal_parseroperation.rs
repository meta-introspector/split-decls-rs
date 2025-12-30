// Generated macro for Operation (enum)
macro_rules! Depcrate_goal_parserOperation {
() => {
// Module: crate::goal_parser
// Provides: {"Operation"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] # [serde (untagged)] pub enum Operation { FunctionCall (FunctionCallOperation) , Loop (LoopOperation) , Sequence (SequenceOperation) , Switch (SwitchOperation) , Shell (ShellCommandOperation) , # [serde (untagged)] Unknown (toml :: Value) , }
};
}
