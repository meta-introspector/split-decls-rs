// Generated macro for macro_104 (macro)
macro_rules! Depcrate_middleware_loggermacro_104 {
() => {
// Module: crate::middleware::logger
// Provides: {"macro_104"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (target_arch = "wasm32")] { mod wasm ; pub use wasm :: Logger ; } else { mod native ; pub use native :: Logger ; } }
};
}
