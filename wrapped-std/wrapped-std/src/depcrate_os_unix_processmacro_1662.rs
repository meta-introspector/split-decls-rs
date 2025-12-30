// Generated macro for macro_1662 (macro)
macro_rules! Depcrate_os_unix_processmacro_1662 {
() => {
// Module: crate::os::unix::process
// Provides: {"macro_1662"}
// Dependencies: {}
cfg_select ! { any (target_os = "vxworks" , target_os = "espidf" , target_os = "horizon" , target_os = "vita") => { type UserId = u16 ; type GroupId = u16 ; } target_os = "nto" => { type UserId = i32 ; type GroupId = i32 ; } _ => { type UserId = u32 ; type GroupId = u32 ; } }
};
}
