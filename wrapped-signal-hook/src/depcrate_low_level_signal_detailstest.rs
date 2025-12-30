// Generated macro for test (module)
macro_rules! Depcrate_low_level_signal_detailstest {
() => {
// Module: crate::low_level::signal_details
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn existing () { assert_eq ! ("SIGTERM" , signal_name (SIGTERM) . unwrap ()) ; } # [test] fn unknown () { assert ! (signal_name (128) . is_none ()) ; } }
};
}
