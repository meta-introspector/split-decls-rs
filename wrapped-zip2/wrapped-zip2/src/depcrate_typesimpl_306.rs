// Generated macro for impl_306 (impl)
macro_rules! Depcrate_typesimpl_306 {
() => {
// Module: crate::types
// Provides: {"impl_306"}
// Dependencies: {}
# [cfg (feature = "time")] impl TryFrom < OffsetDateTime > for DateTime { type Error = DateTimeRangeError ; fn try_from (dt : OffsetDateTime) -> Result < Self , Self :: Error > { Self :: try_from (PrimitiveDateTime :: new (dt . date () , dt . time ())) } }
};
}
