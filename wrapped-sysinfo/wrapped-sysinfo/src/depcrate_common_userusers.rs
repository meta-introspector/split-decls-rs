// Generated macro for Users (struct)
macro_rules! Depcrate_common_userUsers {
() => {
// Module: crate::common::user
// Provides: {"Users"}
// Dependencies: {}
# [doc = " Interacting with users."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Users;"] # [doc = ""] # [doc = " let mut users = Users::new();"] # [doc = " for user in users.list() {"] # [doc = "     println!(\"{} is in {} groups\", user.name(), user.groups().len());"] # [doc = " }"] # [doc = " ```"] pub struct Users { users : Vec < User > , }
};
}
