// Generated macro for var_os (function)
macro_rules! Depcrate_envvar_os {
() => {
// Module: crate::env
// Provides: {"var_os"}
// Dependencies: {}
# [doc = " Fetches the environment variable `key` from the current process, returning"] # [doc = " [`None`] if the variable isn't set or if there is another error."] # [doc = ""] # [doc = " It may return `None` if the environment variable's name contains"] # [doc = " the equal sign character (`=`) or the NUL character."] # [doc = ""] # [doc = " Note that this function will not check if the environment variable"] # [doc = " is valid Unicode. If you want to have an error on invalid UTF-8,"] # [doc = " use the [`var`] function instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::env;"] # [doc = ""] # [doc = " let key = \"HOME\";"] # [doc = " match env::var_os(key) {"] # [doc = "     Some(val) => println!(\"{key}: {val:?}\"),"] # [doc = "     None => println!(\"{key} is not defined in the environment.\")"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " If expecting a delimited variable (such as `PATH`), [`split_paths`]"] # [doc = " can be used to separate items."] # [must_use] # [stable (feature = "env" , since = "1.0.0")] pub fn var_os < K : AsRef < OsStr > > (key : K) -> Option < OsString > { _var_os (key . as_ref ()) }
};
}
