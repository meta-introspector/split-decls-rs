// Generated macro for DateTime (struct)
macro_rules! Depcrate_typesDateTime {
() => {
// Module: crate::types
// Provides: {"DateTime"}
// Dependencies: {}
# [doc = " A date and time for a given calendar."] # [doc = ""] # [doc = " **The primary definition of this type is in the [`icu_time`](https://docs.rs/icu_time) crate. Other ICU4X crates re-export it for convenience.**"] # [derive (Debug , PartialEq , Eq , Clone , Copy)] # [allow (clippy :: exhaustive_structs)] pub struct DateTime < A : AsCalendar > { # [doc = " The date"] pub date : Date < A > , # [doc = " The time"] pub time : Time , }
};
}
