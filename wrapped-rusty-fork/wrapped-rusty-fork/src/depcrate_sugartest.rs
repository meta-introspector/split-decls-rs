// Generated macro for test (module)
macro_rules! Depcrate_sugartest {
() => {
// Module: crate::sugar
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] fn ids_are_actually_distinct () { assert_ne ! (rusty_fork_id ! () , rusty_fork_id ! ()) ; } }
};
}
