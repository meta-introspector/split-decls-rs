// Generated macro for vars_os (function)
macro_rules! Depcrate_envvars_os {
() => {
// Module: crate::env
// Provides: {"vars_os"}
// Dependencies: {}
# [doc = " Returns an iterator of (variable, value) pairs of OS strings, for all the"] # [doc = " environment variables of the current process."] # [doc = ""] # [doc = " The returned iterator contains a snapshot of the process's environment"] # [doc = " variables at the time of this invocation. Modifications to environment"] # [doc = " variables afterwards will not be reflected in the returned iterator."] # [doc = ""] # [doc = " Note that the returned iterator will not check if the environment variables"] # [doc = " are valid Unicode. If you want to panic on invalid UTF-8,"] # [doc = " use the [`vars`] function instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " // Print all environment variables."] # [doc = " for (key, value) in std::env::vars_os() {"] # [doc = "     println!(\"{key:?}: {value:?}\");"] # [doc = " }"] # [doc = " ```"] # [must_use] # [stable (feature = "env" , since = "1.0.0")] pub fn vars_os () -> VarsOs { VarsOs { inner : env_imp :: env () } }
};
}
