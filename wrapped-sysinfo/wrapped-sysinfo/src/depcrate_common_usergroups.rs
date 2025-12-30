// Generated macro for Groups (struct)
macro_rules! Depcrate_common_userGroups {
() => {
// Module: crate::common::user
// Provides: {"Groups"}
// Dependencies: {}
# [doc = " Interacting with groups."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Groups;"] # [doc = ""] # [doc = " let mut groups = Groups::new();"] # [doc = " for group in groups.list() {"] # [doc = "     println!(\"{}\", group.name());"] # [doc = " }"] # [doc = " ```"] pub struct Groups { groups : Vec < Group > , }
};
}
