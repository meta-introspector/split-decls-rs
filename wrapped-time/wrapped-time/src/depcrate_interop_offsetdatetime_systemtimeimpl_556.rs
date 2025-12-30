// Generated macro for impl_556 (impl)
macro_rules! Depcrate_interop_offsetdatetime_systemtimeimpl_556 {
() => {
// Module: crate::interop::offsetdatetime_systemtime
// Provides: {"impl_556"}
// Dependencies: {}
impl PartialOrd < SystemTime > for OffsetDateTime { # [inline] fn partial_cmp (& self , other : & SystemTime) -> Option < Ordering > { self . partial_cmp (& Self :: from (* other)) } }
};
}
