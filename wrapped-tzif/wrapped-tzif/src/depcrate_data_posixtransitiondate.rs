// Generated macro for TransitionDate (struct)
macro_rules! Depcrate_data_posixTransitionDate {
() => {
// Module: crate::data::posix
// Provides: {"TransitionDate"}
// Dependencies: {}
# [doc = " A struct to hold a DST transition date."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct TransitionDate { # [doc = " The day on which the transition occurrs."] pub day : TransitionDay , # [doc = " The time in seconds in which the transition occurrs."] pub time : Seconds , }
};
}
