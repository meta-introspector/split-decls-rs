// Generated macro for impl_192 (impl)
macro_rules! Depcrate_timeimpl_192 {
() => {
// Module: crate::time
// Provides: {"impl_192"}
// Dependencies: {}
impl From < DateTime > for Time { fn from (time : DateTime) -> Time { UtcTime :: from_date_time (time) . map (Self :: UtcTime) . unwrap_or_else (| _e | Self :: GeneralTime (GeneralizedTime :: from_date_time (time))) } }
};
}
