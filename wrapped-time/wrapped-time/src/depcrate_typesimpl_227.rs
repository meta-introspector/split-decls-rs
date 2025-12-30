// Generated macro for impl_227 (impl)
macro_rules! Depcrate_typesimpl_227 {
() => {
// Module: crate::types
// Provides: {"impl_227"}
// Dependencies: {}
impl Time { # [doc = " Construct a new [`Time`], without validating that all components are in range"] pub const fn new (hour : Hour , minute : Minute , second : Second , subsecond : Nanosecond) -> Self { Self { hour , minute , second , subsecond , } } # [doc = " Construct a new [`Time`] representing the start of the day (00:00:00.000)"] pub const fn start_of_day () -> Self { Self { hour : Hour (0) , minute : Minute (0) , second : Second (0) , subsecond : Nanosecond (0) , } } # [doc = " Construct a new [`Time`] representing noon (12:00:00.000)"] pub const fn noon () -> Self { Self { hour : Hour (12) , minute : Minute (0) , second : Second (0) , subsecond : Nanosecond (0) , } } # [doc = " Construct a new [`Time`], whilst validating that all components are in range"] pub fn try_new (hour : u8 , minute : u8 , second : u8 , nanosecond : u32) -> Result < Self , RangeError > { Ok (Self { hour : hour . try_into () ? , minute : minute . try_into () ? , second : second . try_into () ? , subsecond : nanosecond . try_into () ? , }) } }
};
}
