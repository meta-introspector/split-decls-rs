// Generated macro for impl_20 (impl)
macro_rules! Depcrate_datetimeimpl_20 {
() => {
// Module: crate::datetime
// Provides: {"impl_20"}
// Dependencies: {}
impl fmt :: Display for Offset { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Z => write ! (f , "Z") , Self :: Custom { mut minutes } => { let mut sign = '+' ; if minutes < 0 { minutes *= - 1 ; sign = '-' ; } let hours = minutes / 60 ; let minutes = minutes % 60 ; write ! (f , "{sign}{hours:02}:{minutes:02}") } } } }
};
}
