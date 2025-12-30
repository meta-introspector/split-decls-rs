// Generated macro for impl_307 (impl)
macro_rules! Depcrate_typesimpl_307 {
() => {
// Module: crate::types
// Provides: {"impl_307"}
// Dependencies: {}
# [cfg (feature = "time")] impl TryFrom < PrimitiveDateTime > for DateTime { type Error = DateTimeRangeError ; fn try_from (dt : PrimitiveDateTime) -> Result < Self , Self :: Error > { Self :: from_date_and_time (dt . year () . try_into () ? , dt . month () . into () , dt . day () , dt . hour () , dt . minute () , dt . second () ,) } }
};
}
