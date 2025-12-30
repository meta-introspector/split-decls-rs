// Generated macro for unix (module)
macro_rules! Depcrateunix {
() => {
// Module: crate
// Provides: {"unix"}
// Dependencies: {}
# [cfg (all (unix , not (target_os = "macos")))] mod unix ;
};
}
