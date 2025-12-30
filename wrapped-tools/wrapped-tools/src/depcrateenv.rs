// Generated macro for Env (struct)
macro_rules! DepcrateEnv {
() => {
// Module: crate
// Provides: {"Env"}
// Dependencies: {}
# [doc = " A utility to set and unset environment variables, while restoring or removing them on drop."] # [derive (Default)] pub struct Env < 'a > { altered_vars : Vec < (& 'a str , Option < OsString >) > , }
};
}
