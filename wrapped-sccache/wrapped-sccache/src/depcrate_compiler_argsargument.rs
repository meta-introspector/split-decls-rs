// Generated macro for Argument (enum)
macro_rules! Depcrate_compiler_argsArgument {
() => {
// Module: crate::compiler::args
// Provides: {"Argument"}
// Dependencies: {}
# [doc = " Representation of a parsed argument"] # [doc = " The type parameter T contains the parsed information for this argument,"] # [doc = " for use during argument handling (typically an enum to allow switching"] # [doc = " on the different kinds of argument). `Flag`s may contain a simple"] # [doc = " variant which influences how to do caching, whereas `WithValue`s could"] # [doc = " be a struct variant with parsed data from the value."] # [derive (PartialEq , Eq , Clone , Debug)] pub enum Argument < T > { # [doc = " Unknown non-flag argument ; e.g. \"foo\""] Raw (OsString) , # [doc = " Unknown flag argument ; e.g. \"-foo\""] UnknownFlag (OsString) , # [doc = " Known flag argument ; e.g. \"-bar\""] Flag (& 'static str , T) , # [doc = " Known argument with a value ; e.g. \"-qux bar\", where the way the"] # [doc = " value is passed is described by the ArgDisposition type."] WithValue (& 'static str , T , ArgDisposition) , }
};
}
