// Generated macro for VarError (enum)
macro_rules! Depcrate_envVarError {
() => {
// Module: crate::env
// Provides: {"VarError"}
// Dependencies: {}
# [doc = " The error type for operations interacting with environment variables."] # [doc = " Possibly returned from [`env::var()`]."] # [doc = ""] # [doc = " [`env::var()`]: var"] # [derive (Debug , PartialEq , Eq , Clone)] # [stable (feature = "env" , since = "1.0.0")] pub enum VarError { # [doc = " The specified environment variable was not present in the current"] # [doc = " process's environment."] # [stable (feature = "env" , since = "1.0.0")] NotPresent , # [doc = " The specified environment variable was found, but it did not contain"] # [doc = " valid unicode data. The found data is returned as a payload of this"] # [doc = " variant."] # [stable (feature = "env" , since = "1.0.0")] NotUnicode (# [stable (feature = "env" , since = "1.0.0")] OsString) , }
};
}
