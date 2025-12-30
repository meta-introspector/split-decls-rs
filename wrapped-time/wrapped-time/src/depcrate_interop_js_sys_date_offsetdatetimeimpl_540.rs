// Generated macro for impl_540 (impl)
macro_rules! Depcrate_interop_js_sys_date_offsetdatetimeimpl_540 {
() => {
// Module: crate::interop::js_sys_date_offsetdatetime
// Provides: {"impl_540"}
// Dependencies: {}
impl From < js_sys :: Date > for OffsetDateTime { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if the timestamp can not be represented."] # [track_caller] fn from (js_date : js_sys :: Date) -> Self { let timestamp_nanos = js_date . get_time () as i128 * Nanosecond :: per_t :: < i128 > (Millisecond) ; Self :: from_unix_timestamp_nanos (timestamp_nanos) . expect ("invalid timestamp: Timestamp cannot fit in range") } }
};
}
