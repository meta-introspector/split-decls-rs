// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " Note: this is intentionally not public."] enum ErrorKind { CurrentDir { err : io :: Error , path : Option < Arc < Path > > } , Var { err : env :: VarError , var : OsString } , ReadFile { err : io :: Error , path : PathBuf } , ReadDir { err : io :: Error , path : PathBuf } , WriteFile { err : io :: Error , path : PathBuf } , CopyFile { err : io :: Error , src : PathBuf , dst : PathBuf } , HardLink { err : io :: Error , src : PathBuf , dst : PathBuf } , CreateDir { err : io :: Error , path : PathBuf } , RemovePath { err : io :: Error , path : PathBuf } , Cmd (CmdError) , }
};
}
