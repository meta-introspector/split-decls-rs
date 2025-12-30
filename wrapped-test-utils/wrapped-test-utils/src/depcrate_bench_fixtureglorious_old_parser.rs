// Generated macro for glorious_old_parser (function)
macro_rules! Depcrate_bench_fixtureglorious_old_parser {
() => {
// Module: crate::bench_fixture
// Provides: {"glorious_old_parser"}
// Dependencies: {}
pub fn glorious_old_parser () -> String { let path = project_root () . join ("bench_data/glorious_old_parser") ; fs :: read_to_string (path) . unwrap () }
};
}
