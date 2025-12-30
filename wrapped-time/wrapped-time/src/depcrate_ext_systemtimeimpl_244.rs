// Generated macro for impl_244 (impl)
macro_rules! Depcrate_ext_systemtimeimpl_244 {
() => {
// Module: crate::ext::systemtime
// Provides: {"impl_244"}
// Dependencies: {}
impl SystemTimeExt for SystemTime { # [inline] fn checked_add_signed (& self , duration : Duration) -> Option < Self > { if duration . is_positive () { self . checked_add (duration . unsigned_abs ()) } else if duration . is_negative () { self . checked_sub (duration . unsigned_abs ()) } else { Some (* self) } } # [inline] fn checked_sub_signed (& self , duration : Duration) -> Option < Self > { if duration . is_positive () { self . checked_sub (duration . unsigned_abs ()) } else if duration . is_negative () { self . checked_add (duration . unsigned_abs ()) } else { Some (* self) } } # [inline] fn signed_duration_since (& self , earlier : Self) -> Duration { match self . duration_since (earlier) { Ok (duration) => duration . try_into () . unwrap_or (Duration :: MAX) , Err (err) => { let seconds = match i64 :: try_from (err . duration () . as_secs ()) { Ok (seconds) => - seconds , Err (_) => return Duration :: MIN , } ; let nanoseconds = - err . duration () . subsec_nanos () . cast_signed () ; unsafe { Duration :: new_unchecked (seconds , nanoseconds) } } } } }
};
}
