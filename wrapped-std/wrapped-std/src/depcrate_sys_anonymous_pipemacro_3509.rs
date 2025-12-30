// Generated macro for macro_3509 (macro)
macro_rules! Depcrate_sys_anonymous_pipemacro_3509 {
() => {
// Module: crate::sys::anonymous_pipe
// Provides: {"macro_3509"}
// Dependencies: {}
cfg_select ! { unix => { mod unix ; pub use unix :: { AnonPipe , pipe } ; } windows => { mod windows ; pub use windows :: { AnonPipe , pipe } ; } _ => { mod unsupported ; pub use unsupported :: { AnonPipe , pipe } ; } }
};
}
