// Generated macro for Uptime (struct)
macro_rules! Depcrate_timeUptime {
() => {
// Module: crate::time
// Provides: {"Uptime"}
// Dependencies: {}
# [doc = " Retrieve and print the relative elapsed wall-clock time since an epoch."] # [doc = ""] # [doc = " The `Default` implementation for `Uptime` makes the epoch the current time."] # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct Uptime { epoch : std :: time :: Instant , # [doc = " Whether to print the time with higher precision."] pub higher_precision : bool , }
};
}
