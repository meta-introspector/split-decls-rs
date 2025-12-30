// Generated macro for impl_231 (impl)
macro_rules! Depcrate_typesimpl_231 {
() => {
// Module: crate::types
// Provides: {"impl_231"}
// Dependencies: {}
impl Ord for ZonedDateTime < Iso , UtcOffset > { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . to_epoch_milliseconds_utc () . cmp (& other . to_epoch_milliseconds_utc ()) } }
};
}
