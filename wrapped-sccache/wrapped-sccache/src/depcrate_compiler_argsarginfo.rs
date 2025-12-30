// Generated macro for ArgInfo (enum)
macro_rules! Depcrate_compiler_argsArgInfo {
() => {
// Module: crate::compiler::args
// Provides: {"ArgInfo"}
// Dependencies: {}
# [doc = " The description of how an argument may be parsed"] # [derive (PartialEq , Eq , Clone , Debug)] # [allow (unpredictable_function_pointer_comparisons)] pub enum ArgInfo < T > { # [doc = " An simple flag argument, of the form \"-foo\""] Flag (& 'static str , T) , # [doc = " An argument with a value ; e.g. \"-qux bar\", where the way the"] # [doc = " value is passed is described by the ArgDisposition type."] TakeArg (& 'static str , fn (OsString) -> ArgParseResult < T > , ArgDisposition ,) , }
};
}
