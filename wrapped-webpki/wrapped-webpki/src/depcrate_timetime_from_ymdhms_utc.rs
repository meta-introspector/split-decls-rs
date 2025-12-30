// Generated macro for time_from_ymdhms_utc (function)
macro_rules! Depcrate_timetime_from_ymdhms_utc {
() => {
// Module: crate::time
// Provides: {"time_from_ymdhms_utc"}
// Dependencies: {}
pub (crate) fn time_from_ymdhms_utc (year : u64 , month : u64 , day_of_month : u64 , hours : u64 , minutes : u64 , seconds : u64 ,) -> Result < UnixTime , Error > { let days_before_year_since_unix_epoch = days_before_year_since_unix_epoch (year) ? ; const JAN : u64 = 31 ; let feb = days_in_feb (year) ; const MAR : u64 = 31 ; const APR : u64 = 30 ; const MAY : u64 = 31 ; const JUN : u64 = 30 ; const JUL : u64 = 31 ; const AUG : u64 = 31 ; const SEP : u64 = 30 ; const OCT : u64 = 31 ; const NOV : u64 = 30 ; let days_before_month_in_year = match month { 1 => 0 , 2 => JAN , 3 => JAN + feb , 4 => JAN + feb + MAR , 5 => JAN + feb + MAR + APR , 6 => JAN + feb + MAR + APR + MAY , 7 => JAN + feb + MAR + APR + MAY + JUN , 8 => JAN + feb + MAR + APR + MAY + JUN + JUL , 9 => JAN + feb + MAR + APR + MAY + JUN + JUL + AUG , 10 => JAN + feb + MAR + APR + MAY + JUN + JUL + AUG + SEP , 11 => JAN + feb + MAR + APR + MAY + JUN + JUL + AUG + SEP + OCT , 12 => JAN + feb + MAR + APR + MAY + JUN + JUL + AUG + SEP + OCT + NOV , _ => unreachable ! () , } ; let days_before = days_before_year_since_unix_epoch + days_before_month_in_year + day_of_month - 1 ; let seconds_since_unix_epoch = (days_before * 24 * 60 * 60) + (hours * 60 * 60) + (minutes * 60) + seconds ; Ok (UnixTime :: since_unix_epoch (Duration :: from_secs (seconds_since_unix_epoch ,))) }
};
}
