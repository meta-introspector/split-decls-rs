// Generated macro for impl_545 (impl)
macro_rules! Depcrate_interop_js_sys_date_utcdatetimeimpl_545 {
() => {
// Module: crate::interop::js_sys_date_utcdatetime
// Provides: {"impl_545"}
// Dependencies: {}
impl From < js_sys :: Date > for UtcDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if the timestamp can not be represented."] # [track_caller] fn from (js_date : js_sys :: Date) -> Self { let timestamp_nanos = (js_date . get_time () * Nanosecond :: per_t :: < f64 > (Millisecond)) as i128 ; Self :: from_unix_timestamp_nanos (timestamp_nanos) . expect ("invalid timestamp: Timestamp cannot fit in range") } }
};
}
