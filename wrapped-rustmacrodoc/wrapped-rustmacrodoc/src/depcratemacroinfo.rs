// Generated macro for MacroInfo (struct)
macro_rules! DepcrateMacroInfo {
() => {
// Module: crate
// Provides: {"MacroInfo"}
// Dependencies: {}
# [doc = " Represents information about a single macro."] # [derive (Debug , Serialize , Deserialize)] struct MacroInfo { name : String , kind : String , file : String , line : usize , column : usize , # [serde (skip_serializing_if = "Option::is_none")] signature : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] doc_comment : Option < String > , }
};
}
