// Generated macro for DistType (enum)
macro_rules! Depcrate_compiler_compilerDistType {
() => {
// Module: crate::compiler::compiler
// Provides: {"DistType"}
// Dependencies: {}
# [doc = " Specifics about distributed compilation."] # [derive (Debug , PartialEq , Eq)] pub enum DistType { # [doc = " Distribution was not enabled."] NoDist , # [doc = " Distributed compile success."] Ok (dist :: ServerId) , # [doc = " Distributed compile failed."] Error , }
};
}
