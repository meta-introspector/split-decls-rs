// Generated macro for numerous_macro_rules (function)
macro_rules! Depcrate_bench_fixturenumerous_macro_rules {
() => {
// Module: crate::bench_fixture
// Provides: {"numerous_macro_rules"}
// Dependencies: {}
pub fn numerous_macro_rules () -> String { let path = project_root () . join ("bench_data/numerous_macro_rules") ; fs :: read_to_string (path) . unwrap () }
};
}
