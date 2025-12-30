// Generated macro for impl_899 (impl)
macro_rules! Depcrate_features_gen_CheckerboardReportimpl_899 {
() => {
// Module: crate::features::gen_CheckerboardReport
// Provides: {"impl_899"}
// Dependencies: {}
impl CheckerboardReport { # [doc = "Construct a new `CheckerboardReport`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_log()` instead."] pub fn log (& mut self , val : & str) -> & mut Self { self . set_log (val) ; self } # [cfg (feature = "CheckerboardReason")] # [deprecated = "Use `set_reason()` instead."] pub fn reason (& mut self , val : CheckerboardReason) -> & mut Self { self . set_reason (val) ; self } # [deprecated = "Use `set_severity()` instead."] pub fn severity (& mut self , val : u32) -> & mut Self { self . set_severity (val) ; self } # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } }
};
}
