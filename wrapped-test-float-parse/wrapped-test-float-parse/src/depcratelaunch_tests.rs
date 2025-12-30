// Generated macro for launch_tests (function)
macro_rules! Depcratelaunch_tests {
() => {
// Module: crate
// Provides: {"launch_tests"}
// Dependencies: {}
# [doc = " Run all tests in `tests`."] # [doc = ""] # [doc = " This launches a main thread that receives messages and handlees UI updates, and uses the"] # [doc = " rest of the thread pool to execute the tests."] fn launch_tests (tests : & mut [TestInfo] , cfg : & Config) -> Duration { tests . sort_unstable_by_key (| test | (test . total_tests , test . float_bits)) ; for test in tests . iter () { println ! ("Launching test '{}'" , test . name) ; } let mut all_progress_bars = Vec :: new () ; let start = Instant :: now () ; for test in tests . iter_mut () { test . progress = Some (ui :: Progress :: new (test , & mut all_progress_bars)) ; ui :: set_panic_hook (& all_progress_bars) ; (test . launch) (test , cfg) ; } start . elapsed () }
};
}
