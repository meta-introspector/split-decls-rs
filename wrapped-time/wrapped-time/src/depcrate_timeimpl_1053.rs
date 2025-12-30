// Generated macro for impl_1053 (impl)
macro_rules! Depcrate_timeimpl_1053 {
() => {
// Module: crate::time
// Provides: {"impl_1053"}
// Dependencies: {}
impl Sub for Time { type Output = Duration ; # [doc = " Subtract two `Time`s, returning the [`Duration`] between. This assumes both `Time`s are in"] # [doc = " the same calendar day."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::ext::NumericalDuration;"] # [doc = " # use time_macros::time;"] # [doc = " assert_eq!(time!(0:00) - time!(0:00), 0.seconds());"] # [doc = " assert_eq!(time!(1:00) - time!(0:00), 1.hours());"] # [doc = " assert_eq!(time!(0:00) - time!(1:00), (-1).hours());"] # [doc = " assert_eq!(time!(0:00) - time!(23:00), (-23).hours());"] # [doc = " ```"] # [inline] fn sub (self , rhs : Self) -> Self :: Output { let hour_diff = self . hour . get () . cast_signed () - rhs . hour . get () . cast_signed () ; let minute_diff = self . minute . get () . cast_signed () - rhs . minute . get () . cast_signed () ; let second_diff = self . second . get () . cast_signed () - rhs . second . get () . cast_signed () ; let nanosecond_diff = self . nanosecond . get () . cast_signed () - rhs . nanosecond . get () . cast_signed () ; let seconds = hour_diff . extend :: < i32 > () * Second :: per_t :: < i32 > (Hour) + minute_diff . extend :: < i32 > () * Second :: per_t :: < i32 > (Minute) + second_diff . extend :: < i32 > () ; let (seconds , nanoseconds) = if seconds > 0 && nanosecond_diff < 0 { (seconds - 1 , nanosecond_diff + Nanosecond :: per_t :: < i32 > (Second) ,) } else if seconds < 0 && nanosecond_diff > 0 { (seconds + 1 , nanosecond_diff - Nanosecond :: per_t :: < i32 > (Second) ,) } else { (seconds , nanosecond_diff) } ; unsafe { Duration :: new_unchecked (seconds . extend () , nanoseconds) } } }
};
}
