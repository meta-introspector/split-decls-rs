// Generated macro for tests (module)
macro_rules! Depcrate_common_disktests {
() => {
// Module: crate::common::disk
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [doc = " This first doctest ensure that we can create a new `Disks`."] # [doc = ""] # [doc = " ```"] # [doc = " let x = sysinfo::Disks::new();"] # [doc = " ```"] # [doc = ""] # [doc = " This second doctest ensures that `Disks` doesn't implement `Clone`."] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " let x = sysinfo::Disks::new();"] # [doc = " x.clone();"] # [doc = " ```"] # [test] fn check_if_disks_is_send () { fn is_send < T : Send > (_ : & T) { } let disks = crate :: Disks :: new () ; is_send (& disks) ; } }
};
}
