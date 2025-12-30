// Generated macro for ArgDisposition (enum)
macro_rules! Depcrate_compiler_argsArgDisposition {
() => {
// Module: crate::compiler::args
// Provides: {"ArgDisposition"}
// Dependencies: {}
# [doc = " How a value is passed to an argument with a value."] # [derive (PartialEq , Eq , Clone , Debug)] pub enum ArgDisposition { # [doc = " As \"-arg value\""] Separated , # [doc = " As \"-arg value\", but \"-arg<delimiter>value\" would be valid too"] CanBeConcatenated (Delimiter) , # [doc = " As \"-arg<delimiter>value\", but \"-arg value\" would be valid too"] CanBeSeparated (Delimiter) , # [doc = " As \"-arg<delimiter>value\""] Concatenated (Delimiter) , }
};
}
