// Generated macro for Compile (struct)
macro_rules! Depcrate_protocolCompile {
() => {
// Module: crate::protocol
// Provides: {"Compile"}
// Dependencies: {}
# [doc = " The contents of a compile request from a client."] # [derive (Serialize , Deserialize , Debug)] pub struct Compile { # [doc = " The full path to the compiler executable."] pub exe : OsString , # [doc = " The current working directory in which to execute the compile."] pub cwd : OsString , # [doc = " The commandline arguments passed to the compiler."] pub args : Vec < OsString > , # [doc = " The environment variables present when the compiler was executed, as (var, val)."] pub env_vars : Vec < (OsString , OsString) > , }
};
}
