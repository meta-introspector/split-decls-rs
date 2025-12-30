// Generated macro for KEY_WRITE (const)
macro_rules! Depcrate_um_winntKEY_WRITE {
() => {
// Module: crate::um::winnt
// Provides: {"KEY_WRITE"}
// Dependencies: {}
pub const KEY_WRITE : u32 = (STANDARD_RIGHTS_WRITE | KEY_SET_VALUE | KEY_CREATE_SUB_KEY) & ! SYNCHRONIZE ;
};
}
