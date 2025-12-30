// Generated macro for Time (struct)
macro_rules! Depcrate_timeTime {
() => {
// Module: crate::time
// Provides: {"Time"}
// Dependencies: {}
# [doc = " The clock time within a given date. Nanosecond precision."] # [doc = ""] # [doc = " All minutes are assumed to have exactly 60 seconds; no attempt is made to handle leap seconds"] # [doc = " (either positive or negative)."] # [doc = ""] # [doc = " When comparing two `Time`s, they are assumed to be in the same calendar date."] # [derive (Clone , Copy , Eq)] # [cfg_attr (not (docsrs) , repr (C))] pub struct Time { # [cfg (target_endian = "little")] nanosecond : Nanoseconds , # [cfg (target_endian = "little")] second : Seconds , # [cfg (target_endian = "little")] minute : Minutes , # [cfg (target_endian = "little")] hour : Hours , # [cfg (target_endian = "little")] padding : Padding , # [cfg (target_endian = "big")] padding : Padding , # [cfg (target_endian = "big")] hour : Hours , # [cfg (target_endian = "big")] minute : Minutes , # [cfg (target_endian = "big")] second : Seconds , # [cfg (target_endian = "big")] nanosecond : Nanoseconds , }
};
}
