// Generated macro for macro_509 (macro)
macro_rules! Depcrate_reconnect_futuremacro_509 {
() => {
// Module: crate::reconnect::future
// Provides: {"macro_509"}
// Dependencies: {}
pin_project ! { # [project = InnerProj] # [derive (Debug)] enum Inner < F , E > { Future { # [pin] fut : F , } , Error { error : Option < E >, } , } }
};
}
