// Generated macro for User (struct)
macro_rules! Depcrate_common_userUser {
() => {
// Module: crate::common::user
// Provides: {"User"}
// Dependencies: {}
# [doc = " Type containing user information."] # [doc = ""] # [doc = " It is returned by [`Users`][crate::Users]."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Users;"] # [doc = ""] # [doc = " let users = Users::new_with_refreshed_list();"] # [doc = " for user in users.list() {"] # [doc = "     println!(\"{:?}\", user);"] # [doc = " }"] # [doc = " ```"] pub struct User { pub (crate) inner : UserInner , }
};
}
