// Generated macro for impl_557 (impl)
macro_rules! Depcrate_interop_offsetdatetime_systemtimeimpl_557 {
() => {
// Module: crate::interop::offsetdatetime_systemtime
// Provides: {"impl_557"}
// Dependencies: {}
impl PartialOrd < OffsetDateTime > for SystemTime { # [inline] fn partial_cmp (& self , other : & OffsetDateTime) -> Option < Ordering > { OffsetDateTime :: from (* self) . partial_cmp (other) } }
};
}
