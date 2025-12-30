// Generated macro for macro_319 (macro)
macro_rules! Depcrate_blockingmacro_319 {
() => {
// Module: crate::blocking
// Provides: {"macro_319"}
// Dependencies: {}
cfg_rt ! { pub (crate) use crate :: runtime :: spawn_blocking ; cfg_fs ! { # [allow (unused_imports)] pub (crate) use crate :: runtime :: spawn_mandatory_blocking ; } pub (crate) use crate :: task :: JoinHandle ; }
};
}
