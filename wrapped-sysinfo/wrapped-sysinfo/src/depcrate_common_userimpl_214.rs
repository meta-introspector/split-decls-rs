// Generated macro for impl_214 (impl)
macro_rules! Depcrate_common_userimpl_214 {
() => {
// Module: crate::common::user
// Provides: {"impl_214"}
// Dependencies: {}
impl Group { # [doc = " Returns the ID of the group."] # [doc = ""] # [doc = " ⚠\u{fe0f} This information is not set on Windows."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Users;"] # [doc = ""] # [doc = " let mut users = Users::new_with_refreshed_list();"] # [doc = ""] # [doc = " for user in users.list() {"] # [doc = "     for group in user.groups() {"] # [doc = "         println!(\"{:?}\", group.id());"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub fn id (& self) -> & Gid { self . inner . id () } # [doc = " Returns the name of the group."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Users;"] # [doc = ""] # [doc = " let mut users = Users::new_with_refreshed_list();"] # [doc = ""] # [doc = " for user in users.list() {"] # [doc = "     for group in user.groups() {"] # [doc = "         println!(\"{}\", group.name());"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub fn name (& self) -> & str { self . inner . name () } }
};
}
