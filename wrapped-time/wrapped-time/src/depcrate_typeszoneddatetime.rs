// Generated macro for ZonedDateTime (struct)
macro_rules! Depcrate_typesZonedDateTime {
() => {
// Module: crate::types
// Provides: {"ZonedDateTime"}
// Dependencies: {}
# [doc = " A date and time for a given calendar, local to a specified time zone."] # [doc = ""] # [doc = " **The primary definition of this type is in the [`icu_time`](https://docs.rs/icu_time) crate. Other ICU4X crates re-export it for convenience.**"] # [derive (Debug , PartialEq , Eq , Clone , Copy)] # [allow (clippy :: exhaustive_structs)] pub struct ZonedDateTime < A : AsCalendar , Z > { # [doc = " The date, local to the time zone"] pub date : Date < A > , # [doc = " The time, local to the time zone"] pub time : Time , # [doc = " The time zone"] pub zone : Z , }
};
}
