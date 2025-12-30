// Generated macro for get_time (function)
macro_rules! Depcrateget_time {
() => {
// Module: crate
// Provides: {"get_time"}
// Dependencies: {}
fn get_time (millis : u64) -> String { let d = UNIX_EPOCH + Duration :: from_secs (millis) ; let datetime = DateTime :: < Utc > :: from (d) ; datetime . format ("%H:%M:%S") . to_string () }
};
}
