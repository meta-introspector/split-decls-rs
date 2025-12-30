// Generated macro for impl_288 (impl)
macro_rules! Depcrate_timeimpl_288 {
() => {
// Module: crate::time
// Provides: {"impl_288"}
// Dependencies: {}
impl TimeThreshold { # [doc = " Creates a new `TimeThreshold` instance with provided durations."] pub fn new (warn : Duration , critical : Duration) -> Self { Self { warn , critical } } # [doc = " Attempts to create a `TimeThreshold` instance with values obtained"] # [doc = " from the environment variable, and returns `None` if the variable"] # [doc = " is not set."] # [doc = " Environment variable format is expected to match `\\d+,\\d+`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if variable with provided name is set but contains inappropriate"] # [doc = " value."] pub fn from_env_var (env_var_name : & str) -> Option < Self > { let durations_str = env :: var (env_var_name) . ok () ? ; let (warn_str , critical_str) = durations_str . split_once (',') . unwrap_or_else (| | { panic ! ("Duration variable {env_var_name} expected to have 2 numbers separated by comma, but got {durations_str}") }) ; let parse_u64 = | v | { u64 :: from_str (v) . unwrap_or_else (| _ | { panic ! ("Duration value in variable {env_var_name} is expected to be a number, but got {v}") }) } ; let warn = parse_u64 (warn_str) ; let critical = parse_u64 (critical_str) ; if warn > critical { panic ! ("Test execution warn time should be less or equal to the critical time") ; } Some (Self :: new (Duration :: from_millis (warn) , Duration :: from_millis (critical))) } }
};
}
