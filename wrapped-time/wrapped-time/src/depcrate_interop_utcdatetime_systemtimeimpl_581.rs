// Generated macro for impl_581 (impl)
macro_rules! Depcrate_interop_utcdatetime_systemtimeimpl_581 {
() => {
// Module: crate::interop::utcdatetime_systemtime
// Provides: {"impl_581"}
// Dependencies: {}
impl PartialOrd < SystemTime > for UtcDateTime { # [inline] fn partial_cmp (& self , other : & SystemTime) -> Option < Ordering > { self . partial_cmp (& Self :: from (* other)) } }
};
}
