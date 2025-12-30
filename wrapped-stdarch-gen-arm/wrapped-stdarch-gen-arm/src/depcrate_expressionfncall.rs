// Generated macro for FnCall (struct)
macro_rules! Depcrate_expressionFnCall {
() => {
// Module: crate::expression
// Provides: {"FnCall"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct FnCall (# [doc = " Function pointer"] pub Box < Expression > , # [doc = " Function arguments"] pub Vec < Expression > , # [doc = " Function turbofish arguments"] # [serde (default)] pub Vec < Expression > , # [doc = " Function requires unsafe wrapper"] # [serde (default)] pub bool ,) ;
};
}
