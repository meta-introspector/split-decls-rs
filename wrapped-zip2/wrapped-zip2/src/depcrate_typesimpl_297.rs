// Generated macro for impl_297 (impl)
macro_rules! Depcrate_typesimpl_297 {
() => {
// Module: crate::types
// Provides: {"impl_297"}
// Dependencies: {}
# [cfg (feature = "chrono")] impl TryFrom < NaiveDateTime > for DateTime { type Error = DateTimeRangeError ; fn try_from (value : NaiveDateTime) -> Result < Self , Self :: Error > { DateTime :: from_date_and_time (value . year () . try_into () ? , value . month () . try_into () ? , value . day () . try_into () ? , value . hour () . try_into () ? , value . minute () . try_into () ? , value . second () . try_into () ? ,) } }
};
}
