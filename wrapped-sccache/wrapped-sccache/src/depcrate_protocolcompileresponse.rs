// Generated macro for CompileResponse (enum)
macro_rules! Depcrate_protocolCompileResponse {
() => {
// Module: crate::protocol
// Provides: {"CompileResponse"}
// Dependencies: {}
# [doc = " Possible responses from the server for a `Compile` request."] # [derive (Serialize , Deserialize , Debug)] pub enum CompileResponse { # [doc = " The compilation was started."] CompileStarted , # [doc = " The server could not handle this compilation request."] UnhandledCompile , # [doc = " The compiler was not supported."] UnsupportedCompiler (OsString) , }
};
}
