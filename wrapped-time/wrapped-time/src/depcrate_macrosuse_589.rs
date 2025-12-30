// Generated macro for use_589 (pub_use)
macro_rules! Depcrate_macrosuse_589 {
() => {
// Module: crate::macros
// Provides: {"use_589"}
// Dependencies: {}
# [doc = " Construct a [`UtcOffset`](crate::UtcOffset) with a statically known value."] # [doc = ""] # [doc = " The resulting expression can be used in `const` or `static` declarations."] # [doc = ""] # [doc = " A sign and the hour must be provided; minutes and seconds default to zero. `UTC` (both"] # [doc = " uppercase and lowercase) is also allowed."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::{UtcOffset, macros::offset};"] # [doc = " assert_eq!(offset!(UTC), UtcOffset::from_hms(0, 0, 0)?);"] # [doc = " assert_eq!(offset!(utc), UtcOffset::from_hms(0, 0, 0)?);"] # [doc = " assert_eq!(offset!(+0), UtcOffset::from_hms(0, 0, 0)?);"] # [doc = " assert_eq!(offset!(+1), UtcOffset::from_hms(1, 0, 0)?);"] # [doc = " assert_eq!(offset!(-1), UtcOffset::from_hms(-1, 0, 0)?);"] # [doc = " assert_eq!(offset!(+1:30), UtcOffset::from_hms(1, 30, 0)?);"] # [doc = " assert_eq!(offset!(-1:30), UtcOffset::from_hms(-1, -30, 0)?);"] # [doc = " assert_eq!(offset!(+1:30:59), UtcOffset::from_hms(1, 30, 59)?);"] # [doc = " assert_eq!(offset!(-1:30:59), UtcOffset::from_hms(-1, -30, -59)?);"] # [doc = " assert_eq!(offset!(+23:59:59), UtcOffset::from_hms(23, 59, 59)?);"] # [doc = " assert_eq!(offset!(-23:59:59), UtcOffset::from_hms(-23, -59, -59)?);"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] pub use time_macros :: offset ;
};
}
