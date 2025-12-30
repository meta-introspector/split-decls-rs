// Generated macro for FileDesc (enum)
macro_rules! Depcrate_astFileDesc {
() => {
// Module: crate::ast
// Provides: {"FileDesc"}
// Dependencies: {}
# [doc = " File descriptor that describes the file handle manipulated"] # [doc = " by a redirection."] # [derive (Debug , From , PartialEq)] pub (crate) enum FileDesc { Number , StdOutErr , Var (String) , }
};
}
