// Generated macro for impl_582 (impl)
macro_rules! Depcrate_interop_utcdatetime_systemtimeimpl_582 {
() => {
// Module: crate::interop::utcdatetime_systemtime
// Provides: {"impl_582"}
// Dependencies: {}
impl PartialOrd < UtcDateTime > for SystemTime { # [inline] fn partial_cmp (& self , other : & UtcDateTime) -> Option < Ordering > { UtcDateTime :: from (* self) . partial_cmp (other) } }
};
}
