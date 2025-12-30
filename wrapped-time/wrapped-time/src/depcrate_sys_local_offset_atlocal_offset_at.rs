// Generated macro for local_offset_at (function)
macro_rules! Depcrate_sys_local_offset_atlocal_offset_at {
() => {
// Module: crate::sys::local_offset_at
// Provides: {"local_offset_at"}
// Dependencies: {}
# [doc = " Attempt to obtain the system's UTC offset. If the offset cannot be determined, `None` is"] # [doc = " returned."] # [inline] pub (crate) fn local_offset_at (datetime : OffsetDateTime) -> Option < UtcOffset > { imp :: local_offset_at (datetime) }
};
}
