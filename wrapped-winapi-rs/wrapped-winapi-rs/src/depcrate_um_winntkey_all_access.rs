// Generated macro for KEY_ALL_ACCESS (const)
macro_rules! Depcrate_um_winntKEY_ALL_ACCESS {
() => {
// Module: crate::um::winnt
// Provides: {"KEY_ALL_ACCESS"}
// Dependencies: {}
pub const KEY_ALL_ACCESS : u32 = (STANDARD_RIGHTS_ALL | KEY_QUERY_VALUE | KEY_SET_VALUE | KEY_CREATE_SUB_KEY | KEY_ENUMERATE_SUB_KEYS | KEY_NOTIFY | KEY_CREATE_LINK) & ! SYNCHRONIZE ;
};
}
