// Generated macro for HUGE_TEST_CUTOFF (const)
macro_rules! DepcrateHUGE_TEST_CUTOFF {
() => {
// Module: crate
// Provides: {"HUGE_TEST_CUTOFF"}
// Dependencies: {}
# [doc = " If there are more tests than this threshold, the test will be deferred until after all"] # [doc = " others run (so as to avoid thread pool starvation). They also can be excluded with"] # [doc = " `--skip-huge`."] const HUGE_TEST_CUTOFF : u64 = 5_000_000 ;
};
}
