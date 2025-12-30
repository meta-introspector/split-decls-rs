// Generated macro for var (function)
macro_rules! Depcrate_envvar {
() => {
// Module: crate::env
// Provides: {"var"}
// Dependencies: {}
# [doc = " Fetches the environment variable `key` from the current process."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns [`VarError::NotPresent`] if:"] # [doc = " - The variable is not set."] # [doc = " - The variable's name contains an equal sign or NUL (`'='` or `'\\0'`)."] # [doc = ""] # [doc = " Returns [`VarError::NotUnicode`] if the variable's value is not valid"] # [doc = " Unicode. If this is not desired, consider using [`var_os`]."] # [doc = ""] # [doc = " Use [`env!`] or [`option_env!`] instead if you want to check environment"] # [doc = " variables at compile time."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::env;"] # [doc = ""] # [doc = " let key = \"HOME\";"] # [doc = " match env::var(key) {"] # [doc = "     Ok(val) => println!(\"{key}: {val:?}\"),"] # [doc = "     Err(e) => println!(\"couldn't interpret {key}: {e}\"),"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "env" , since = "1.0.0")] pub fn var < K : AsRef < OsStr > > (key : K) -> Result < String , VarError > { _var (key . as_ref ()) }
};
}
