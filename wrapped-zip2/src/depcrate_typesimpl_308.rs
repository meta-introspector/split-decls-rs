// Generated macro for impl_308 (impl)
macro_rules! Depcrate_typesimpl_308 {
() => {
// Module: crate::types
// Provides: {"impl_308"}
// Dependencies: {}
# [cfg (feature = "time")] impl TryFrom < DateTime > for OffsetDateTime { type Error = ComponentRange ; fn try_from (dt : DateTime) -> Result < Self , Self :: Error > { PrimitiveDateTime :: try_from (dt) . map (PrimitiveDateTime :: assume_utc) } }
};
}
