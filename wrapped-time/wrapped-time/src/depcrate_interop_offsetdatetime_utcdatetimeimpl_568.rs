// Generated macro for impl_568 (impl)
macro_rules! Depcrate_interop_offsetdatetime_utcdatetimeimpl_568 {
() => {
// Module: crate::interop::offsetdatetime_utcdatetime
// Provides: {"impl_568"}
// Dependencies: {}
impl PartialOrd < OffsetDateTime > for UtcDateTime { # [inline] fn partial_cmp (& self , other : & OffsetDateTime) -> Option < Ordering > { OffsetDateTime :: from (* self) . partial_cmp (other) } }
};
}
