// Generated macro for Time (struct)
macro_rules! Depcrate_typesTime {
() => {
// Module: crate::types
// Provides: {"Time"}
// Dependencies: {}
# [doc = " A representation of a time in hours, minutes, seconds, and nanoseconds"] # [doc = ""] # [doc = " **The primary definition of this type is in the [`icu_time`](https://docs.rs/icu_time) crate. Other ICU4X crates re-export it for convenience.**"] # [doc = ""] # [doc = " This type supports the range [00:00:00.000000000, 23:59:60.999999999]."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] # [allow (clippy :: exhaustive_structs)] pub struct Time { # [doc = " Hour"] pub hour : Hour , # [doc = " Minute"] pub minute : Minute , # [doc = " Second"] pub second : Second , # [doc = " Subsecond"] pub subsecond : Nanosecond , }
};
}
