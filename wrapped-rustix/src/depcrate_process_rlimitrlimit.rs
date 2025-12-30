// Generated macro for Rlimit (struct)
macro_rules! Depcrate_process_rlimitRlimit {
() => {
// Module: crate::process::rlimit
// Provides: {"Rlimit"}
// Dependencies: {}
# [doc = " `struct rlimit`—Current and maximum values used in [`getrlimit`],"] # [doc = " [`setrlimit`], and [`prlimit`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Rlimit { # [doc = " Current effective, “soft”, limit."] pub current : Option < u64 > , # [doc = " Maximum, “hard”, value that `current` may be dynamically increased to."] pub maximum : Option < u64 > , }
};
}
