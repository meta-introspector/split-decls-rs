// Generated macro for args_os (function)
macro_rules! Depcrate_envargs_os {
() => {
// Module: crate::env
// Provides: {"args_os"}
// Dependencies: {}
# [doc = " Returns the arguments that this program was started with (normally passed"] # [doc = " via the command line)."] # [doc = ""] # [doc = " The first element is traditionally the path of the executable, but it can be"] # [doc = " set to arbitrary text, and might not even exist. This means this property should"] # [doc = " not be relied upon for security purposes."] # [doc = ""] # [doc = " On Unix systems the shell usually expands unquoted arguments with glob patterns"] # [doc = " (such as `*` and `?`). On Windows this is not done, and such arguments are"] # [doc = " passed as-is."] # [doc = ""] # [doc = " On glibc Linux systems, arguments are retrieved by placing a function in `.init_array`."] # [doc = " glibc passes `argc`, `argv`, and `envp` to functions in `.init_array`, as a non-standard"] # [doc = " extension. This allows `std::env::args_os` to work even in a `cdylib` or `staticlib`, as it"] # [doc = " does on macOS and Windows."] # [doc = ""] # [doc = " Note that the returned iterator will not check if the arguments to the"] # [doc = " process are valid Unicode. If you want to panic on invalid UTF-8,"] # [doc = " use the [`args`] function instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::env;"] # [doc = ""] # [doc = " // Prints each argument on a separate line"] # [doc = " for argument in env::args_os() {"] # [doc = "     println!(\"{argument:?}\");"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "env" , since = "1.0.0")] pub fn args_os () -> ArgsOs { ArgsOs { inner : sys :: args :: args () } }
};
}
