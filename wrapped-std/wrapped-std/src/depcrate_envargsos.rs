// Generated macro for ArgsOs (struct)
macro_rules! Depcrate_envArgsOs {
() => {
// Module: crate::env
// Provides: {"ArgsOs"}
// Dependencies: {}
# [doc = " An iterator over the arguments of a process, yielding an [`OsString`] value"] # [doc = " for each argument."] # [doc = ""] # [doc = " This struct is created by [`env::args_os()`]. See its documentation"] # [doc = " for more."] # [doc = ""] # [doc = " The first element is traditionally the path of the executable, but it can be"] # [doc = " set to arbitrary text, and might not even exist. This means this property"] # [doc = " should not be relied upon for security purposes."] # [doc = ""] # [doc = " [`env::args_os()`]: args_os"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "env" , since = "1.0.0")] pub struct ArgsOs { inner : sys :: args :: Args , }
};
}
