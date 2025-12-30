// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug)] pub enum Error { # [doc = " Aborted because of a `VCPKGRS_NO_*` environment variable."] # [doc = ""] # [doc = " Contains the name of the responsible environment variable."] DisabledByEnv (String) , # [doc = " Aborted because a required environment variable was not set."] RequiredEnvMissing (String) , # [doc = " On Windows, only MSVC ABI is supported"] NotMSVC , # [doc = " Can't find a vcpkg tree"] VcpkgNotFound (String) , # [doc = " Library not found in vcpkg tree"] LibNotFound (String) , # [doc = " Could not understand vcpkg installation"] VcpkgInstallation (String) , # [doc (hidden)] __Nonexhaustive , }
};
}
