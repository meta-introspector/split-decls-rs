// Generated macro for impl_1050 (impl)
macro_rules! Depcrate_classify_status_in_range_is_errorimpl_1050 {
() => {
// Module: crate::classify::status_in_range_is_error
// Provides: {"impl_1050"}
// Dependencies: {}
impl StatusInRangeAsFailures { # [doc = " Creates a new `StatusInRangeAsFailures`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the start or end of `range` aren't valid status codes as determined by"] # [doc = " [`StatusCode::from_u16`]."] # [doc = ""] # [doc = " [`StatusCode::from_u16`]: https://docs.rs/http/latest/http/status/struct.StatusCode.html#method.from_u16"] pub fn new (range : RangeInclusive < u16 >) -> Self { assert ! (StatusCode :: from_u16 (* range . start ()) . is_ok () , "range start isn't a valid status code") ; assert ! (StatusCode :: from_u16 (* range . end ()) . is_ok () , "range end isn't a valid status code") ; Self { range } } # [doc = " Creates a new `StatusInRangeAsFailures` that classifies client and server responses as"] # [doc = " failures."] # [doc = ""] # [doc = " This is a convenience for `StatusInRangeAsFailures::new(400..=599)`."] pub fn new_for_client_and_server_errors () -> Self { Self :: new (400 ..= 599) } # [doc = " Convert this `StatusInRangeAsFailures` into a [`MakeClassifier`]."] # [doc = ""] # [doc = " [`MakeClassifier`]: super::MakeClassifier"] pub fn into_make_classifier (self) -> SharedClassifier < Self > { SharedClassifier :: new (self) } }
};
}
