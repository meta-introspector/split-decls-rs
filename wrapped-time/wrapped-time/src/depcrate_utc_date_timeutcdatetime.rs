// Generated macro for UtcDateTime (struct)
macro_rules! Depcrate_utc_date_timeUtcDateTime {
() => {
// Module: crate::utc_date_time
// Provides: {"UtcDateTime"}
// Dependencies: {}
# [doc = " A [`PrimitiveDateTime`] that is known to be UTC."] # [doc = ""] # [doc = " `UtcDateTime` is guaranteed to be ABI-compatible with [`PrimitiveDateTime`], meaning that"] # [doc = " transmuting from one to the other will not result in undefined behavior."] # [repr (transparent)] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct UtcDateTime { inner : PrimitiveDateTime , }
};
}
