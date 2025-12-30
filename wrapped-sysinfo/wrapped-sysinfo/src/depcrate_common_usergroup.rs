// Generated macro for Group (struct)
macro_rules! Depcrate_common_userGroup {
() => {
// Module: crate::common::user
// Provides: {"Group"}
// Dependencies: {}
# [doc = " Type containing group information."] # [doc = ""] # [doc = " It is returned by [`User::groups`] or [`Groups::list`]."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use sysinfo::Users;"] # [doc = ""] # [doc = " let mut users = Users::new_with_refreshed_list();"] # [doc = ""] # [doc = " for user in users.list() {"] # [doc = "     println!("] # [doc = "         \"user: (ID: {:?}, group ID: {:?}, name: {:?})\","] # [doc = "         user.id(),"] # [doc = "         user.group_id(),"] # [doc = "         user.name(),"] # [doc = "     );"] # [doc = "     for group in user.groups() {"] # [doc = "         println!(\"group: (ID: {:?}, name: {:?})\", group.id(), group.name());"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [derive (PartialEq , Eq , PartialOrd , Ord , Debug)] pub struct Group { pub (crate) inner : GroupInner , }
};
}
