// Generated macro for use_187 (pub_use)
macro_rules! Depcrate_deadline_supportuse_187 {
() => {
// Module: crate::deadline_support
// Provides: {"use_187"}
// Dependencies: {}
# [doc = " WASM (browser) specific instant type."] # [doc = ""] # [doc = " This type is only available when the `wasm32_web_time` feature is enabled.  In that"] # [doc = " case this is an alias for [`web_time::Instant`]."] # [cfg (feature = "wasm32_web_time")] pub use web_time :: Instant ;
};
}
