// Generated macro for LeapSecondRecord (struct)
macro_rules! Depcrate_data_tzifLeapSecondRecord {
() => {
// Module: crate::data::tzif
// Provides: {"LeapSecondRecord"}
// Dependencies: {}
# [doc = " A record specifying the corrections that need to be applied to the UTC in"] # [doc = " in order to determine TAI."] # [derive (Debug , Clone , Copy , Default , PartialEq , Eq)] pub struct LeapSecondRecord { # [doc = " A UNIX leap time value"] # [doc = " specifying the time at which a leap-second correction occurs."] # [doc = " The first value, if present, MUST be nonnegative, and each"] # [doc = " later value MUST be at least 2419199 greater than the previous"] # [doc = " value.  (This is 28 days' worth of seconds, minus a potential"] # [doc = " negative leap second.)"] pub occurrence : Seconds , # [doc = " A signed integer specifying the value of"] # [doc = " LEAPCORR on or after the occurrence.  The correction value in"] # [doc = " the first leap-second record, if present, MUST be either one"] # [doc = " (1) or minus one (-1).  The correction values in adjacent leap-"] # [doc = " second records MUST differ by exactly one (1).  The value of"] # [doc = " LEAPCORR is zero for timestamps that occur before the"] # [doc = " occurrence time in the first leap-second record (or for all"] # [doc = " timestamps if there are no leap-second records)."] pub correction : i32 , }
};
}
