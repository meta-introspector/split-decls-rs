// Generated macro for with_var_unset (function)
macro_rules! Depcratewith_var_unset {
() => {
// Module: crate
// Provides: {"with_var_unset"}
// Dependencies: {}
# [doc = " Unsets a single environment variable for the duration of the closure."] # [doc = ""] # [doc = " The previous value is restored when the closure completes or panics, before unwinding the"] # [doc = " panic."] # [doc = ""] # [doc = " This is a shorthand and identical to the following:"] # [doc = " ```rust"] # [doc = " temp_env::with_var(\"MY_ENV_VAR\", None::<&str>, || {"] # [doc = "     // Run some code where `MY_ENV_VAR` is unset."] # [doc = " });"] # [doc = " ```"] pub fn with_var_unset < K , F , R > (key : K , closure : F) -> R where K : AsRef < OsStr > + Clone + Eq + Hash , F : FnOnce () -> R , { with_var (key , None :: < & str > , closure) }
};
}
