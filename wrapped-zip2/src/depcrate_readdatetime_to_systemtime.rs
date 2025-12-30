// Generated macro for datetime_to_systemtime (function)
macro_rules! Depcrate_readdatetime_to_systemtime {
() => {
// Module: crate::read
// Provides: {"datetime_to_systemtime"}
// Dependencies: {}
# [cfg (feature = "chrono")] # [doc = " Generate a `SystemTime` from a `DateTime`."] fn datetime_to_systemtime (datetime : & DateTime) -> Option < std :: time :: SystemTime > { if let Some (t) = generate_chrono_datetime (datetime) { let time = chrono :: DateTime :: < chrono :: Utc > :: from_naive_utc_and_offset (t , chrono :: Utc) ; return Some (time . into ()) ; } None }
};
}
