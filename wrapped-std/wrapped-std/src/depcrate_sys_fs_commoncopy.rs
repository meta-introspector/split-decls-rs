// Generated macro for copy (function)
macro_rules! Depcrate_sys_fs_commoncopy {
() => {
// Module: crate::sys::fs::common
// Provides: {"copy"}
// Dependencies: {}
pub fn copy (from : & Path , to : & Path) -> io :: Result < u64 > { let mut reader = fs :: File :: open (from) ? ; let metadata = reader . metadata () ? ; if ! metadata . is_file () { return Err (NOT_FILE_ERROR) ; } let mut writer = fs :: File :: create (to) ? ; let perm = metadata . permissions () ; let ret = io :: copy (& mut reader , & mut writer) ? ; writer . set_permissions (perm) ? ; Ok (ret) }
};
}
