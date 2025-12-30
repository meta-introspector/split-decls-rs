// Generated macro for KEY_READ (const)
macro_rules! Depcrate_um_winntKEY_READ {
() => {
// Module: crate::um::winnt
// Provides: {"KEY_READ"}
// Dependencies: {}
pub const KEY_READ : u32 = (STANDARD_RIGHTS_READ | KEY_QUERY_VALUE | KEY_ENUMERATE_SUB_KEYS | KEY_NOTIFY) & ! SYNCHRONIZE ;
};
}
