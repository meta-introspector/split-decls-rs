// Generated macro for CompileCommand (struct)
macro_rules! Depcrate_distCompileCommand {
() => {
// Module: crate::dist
// Provides: {"CompileCommand"}
// Dependencies: {}
# [derive (Clone , Debug , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct CompileCommand { pub executable : String , pub arguments : Vec < String > , pub env_vars : Vec < (String , String) > , pub cwd : String , }
};
}
