// Generated macro for CompileFinished (struct)
macro_rules! Depcrate_protocolCompileFinished {
() => {
// Module: crate::protocol
// Provides: {"CompileFinished"}
// Dependencies: {}
# [doc = " Information about a finished compile, either from cache or executed locally."] # [derive (Serialize , Deserialize , Debug , Default)] pub struct CompileFinished { # [doc = " The return code of the compile process, if available."] pub retcode : Option < i32 > , # [doc = " The signal that terminated the compile process, if available."] pub signal : Option < i32 > , # [doc = " The compiler's stdout."] pub stdout : Vec < u8 > , # [doc = " The compiler's stderr."] pub stderr : Vec < u8 > , # [doc = " The state of any compiler options passed to control color output."] pub color_mode : ColorMode , }
};
}
