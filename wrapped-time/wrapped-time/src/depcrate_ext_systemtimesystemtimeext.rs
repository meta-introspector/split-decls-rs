// Generated macro for SystemTimeExt (trait)
macro_rules! Depcrate_ext_systemtimeSystemTimeExt {
() => {
// Module: crate::ext::systemtime
// Provides: {"SystemTimeExt"}
// Dependencies: {}
# [doc = " An extension trait for [`std::time::SystemTime`] that adds methods for"] # [doc = " [`time::Duration`](Duration)s."] pub trait SystemTimeExt : sealed :: Sealed { # [doc = " Adds the given [`Duration`] to the [`SystemTime`], returning `None` is the result cannot be"] # [doc = " represented by the underlying data structure."] fn checked_add_signed (& self , duration : Duration) -> Option < Self > ; # [doc = " Subtracts the given [`Duration`] from the [`SystemTime`], returning `None` is the result"] # [doc = " cannot be represented by the underlying data structure."] fn checked_sub_signed (& self , duration : Duration) -> Option < Self > ; # [doc = " Returns the amount of time elapsed from another [`SystemTime`] to this one. This will be"] # [doc = " negative if `earlier` is later than `self.`"] # [doc = ""] # [doc = " If the duration cannot be stored by [`Duration`], the value will be saturated to"] # [doc = " [`Duration::MIN`] or [`Duration::MAX`] as appropriate."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::time::SystemTime;"] # [doc = " # use time::ext::{NumericalDuration, SystemTimeExt};"] # [doc = " let epoch = SystemTime::UNIX_EPOCH;"] # [doc = " let other = epoch + 1.seconds();"] # [doc = " assert_eq!(other.signed_duration_since(epoch), 1.seconds());"] # [doc = " assert_eq!(epoch.signed_duration_since(other), (-1).seconds());"] # [doc = " ```"] fn signed_duration_since (& self , earlier : Self) -> Duration ; }
};
}
