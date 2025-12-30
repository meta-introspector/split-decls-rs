// Generated macro for macro_315 (macro)
macro_rules! Depcrate_unix_applemacro_315 {
() => {
// Module: crate::unix::apple
// Provides: {"macro_315"}
// Dependencies: {}
cfg_if ! { if # [cfg (all (target_os = "macos" , any (feature = "disk" , feature = "system" , feature = "component")))] { pub (crate) mod macos ; pub (crate) use self :: macos as inner ; } else if # [cfg (all (target_os = "ios" , any (feature = "system" , feature = "component")))] { pub (crate) mod ios ; pub (crate) use self :: ios as inner ; } }
};
}
