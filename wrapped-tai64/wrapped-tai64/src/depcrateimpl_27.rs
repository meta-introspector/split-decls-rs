// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl ops :: Sub < Duration > for Tai64N { type Output = Self ; fn sub (self , d : Duration) -> Self { let (carry , n) = if self . 1 >= d . subsec_nanos () { (0 , self . 1 - d . subsec_nanos ()) } else { (1 , NANOS_PER_SECOND + self . 1 - d . subsec_nanos ()) } ; Tai64N (self . 0 - carry - d . as_secs () , n) } }
};
}
