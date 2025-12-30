// Generated macro for impl_541 (impl)
macro_rules! Depcrate_interop_js_sys_date_offsetdatetimeimpl_541 {
() => {
// Module: crate::interop::js_sys_date_offsetdatetime
// Provides: {"impl_541"}
// Dependencies: {}
impl From < OffsetDateTime > for js_sys :: Date { fn from (datetime : OffsetDateTime) -> Self { let timestamp = (datetime . unix_timestamp_nanos () / Nanosecond :: per_t :: < i128 > (Millisecond)) as f64 ; Self :: new (& timestamp . into ()) } }
};
}
