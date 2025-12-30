// Generated macro for tests (module)
macro_rules! Depcrate_preferencestests {
() => {
// Module: crate::preferences
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn retain_count () { let preferences = SCPreferences :: default (& CFString :: new ("test")) ; assert_eq ! (preferences . retain_count () , 1) ; } }
};
}
