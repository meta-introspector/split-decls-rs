// Generated macro for impl_546 (impl)
macro_rules! Depcrate_interop_js_sys_date_utcdatetimeimpl_546 {
() => {
// Module: crate::interop::js_sys_date_utcdatetime
// Provides: {"impl_546"}
// Dependencies: {}
impl From < UtcDateTime > for js_sys :: Date { fn from (datetime : UtcDateTime) -> Self { let timestamp = (datetime . unix_timestamp_nanos () / Nanosecond :: per_t :: < i128 > (Millisecond)) as f64 ; Self :: new (& timestamp . into ()) } }
};
}
