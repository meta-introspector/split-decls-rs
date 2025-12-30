// Generated macro for impl_74 (impl)
macro_rules! Depcrate_durationimpl_74 {
() => {
// Module: crate::duration
// Provides: {"impl_74"}
// Dependencies: {}
impl SubAssign < Duration > for StdDuration { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if the resulting subtraction can not be represented."] # [inline] # [track_caller] fn sub_assign (& mut self , rhs : Duration) { * self = (* self - rhs) . try_into () . expect ("Cannot represent a resulting duration in std. Try `let x = x - rhs;`, which will \
             change the type." ,) ; } }
};
}
