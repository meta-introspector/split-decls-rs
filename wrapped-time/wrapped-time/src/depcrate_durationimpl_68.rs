// Generated macro for impl_68 (impl)
macro_rules! Depcrate_durationimpl_68 {
() => {
// Module: crate::duration
// Provides: {"impl_68"}
// Dependencies: {}
impl AddAssign < Duration > for StdDuration { # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if the resulting addition cannot be represented."] # [inline] # [track_caller] fn add_assign (& mut self , rhs : Duration) { * self = (* self + rhs) . try_into () . expect ("Cannot represent a resulting duration in std. Try `let x = x + rhs;`, which will \
             change the type." ,) ; } }
};
}
