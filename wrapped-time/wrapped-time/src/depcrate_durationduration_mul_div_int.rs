// Generated macro for duration_mul_div_int (macro)
macro_rules! Depcrate_durationduration_mul_div_int {
() => {
// Module: crate::duration
// Provides: {"duration_mul_div_int"}
// Dependencies: {}
# [doc = " Implement `Mul` (reflexively) and `Div` for `Duration` for various types."] macro_rules ! duration_mul_div_int { ($ ($ type : ty) ,+) => { $ (impl Mul <$ type > for Duration { type Output = Self ; # [inline] # [track_caller] fn mul (self , rhs : $ type) -> Self :: Output { Self :: nanoseconds_i128 (self . whole_nanoseconds () . checked_mul (rhs as i128) . expect ("overflow when multiplying duration")) } } impl Mul < Duration > for $ type { type Output = Duration ; # [inline] # [track_caller] fn mul (self , rhs : Duration) -> Self :: Output { rhs * self } } impl Div <$ type > for Duration { type Output = Self ; # [inline] # [track_caller] fn div (self , rhs : $ type) -> Self :: Output { Self :: nanoseconds_i128 (self . whole_nanoseconds () / (rhs as i128)) } }) + } ; }
};
}
