// Generated macro for LocalDateTime (struct)
macro_rules! Depcrate_timeLocalDateTime {
() => {
// Module: crate::time
// Provides: {"LocalDateTime"}
// Dependencies: {}
# [doc = " Retrieve and print the current wall-clock time."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if [time crate] cannot determine the local UTC offset."] # [doc = ""] # [doc = " [time crate]: time"] # [cfg (feature = "time")] # [derive (Debug , Clone , Copy , Eq , PartialEq , Default)] pub struct LocalDateTime { # [doc = " Whether to print the time with higher precision."] pub higher_precision : bool , }
};
}
