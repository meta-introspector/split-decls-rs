// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
# [allow (clippy :: suspicious_arithmetic_impl)] impl ops :: Add < Duration > for Tai64N { type Output = Self ; fn add (self , d : Duration) -> Self { let n = self . 1 + d . subsec_nanos () ; let (carry , n) = if n >= NANOS_PER_SECOND { (1 , n - NANOS_PER_SECOND) } else { (0 , n) } ; Tai64N (self . 0 + d . as_secs () + carry , n) } }
};
}
