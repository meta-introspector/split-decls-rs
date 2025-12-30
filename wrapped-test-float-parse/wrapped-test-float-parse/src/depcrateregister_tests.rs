// Generated macro for register_tests (function)
macro_rules! Depcrateregister_tests {
() => {
// Module: crate
// Provides: {"register_tests"}
// Dependencies: {}
# [doc = " Enumerate tests to run but don't actually run them."] pub fn register_tests (cfg : & Config) -> Vec < TestInfo > { let mut tests = Vec :: new () ; # [cfg (target_has_reliable_f16)] register_float :: < f16 > (& mut tests , cfg) ; register_float :: < f32 > (& mut tests , cfg) ; register_float :: < f64 > (& mut tests , cfg) ; tests . sort_unstable_by_key (| t | (t . float_name , t . gen_name)) ; for i in 0 .. (tests . len () - 1) { if tests [i] . gen_name == tests [i + 1] . gen_name { panic ! ("duplicate test name {}" , tests [i] . gen_name) ; } } tests }
};
}
