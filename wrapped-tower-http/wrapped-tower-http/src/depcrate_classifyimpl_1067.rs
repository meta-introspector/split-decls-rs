// Generated macro for impl_1067 (impl)
macro_rules! Depcrate_classifyimpl_1067 {
() => {
// Module: crate::classify
// Provides: {"impl_1067"}
// Dependencies: {}
impl ServerErrorsAsFailures { # [doc = " Create a new [`ServerErrorsAsFailures`]."] pub fn new () -> Self { Self :: default () } # [doc = " Returns a [`MakeClassifier`] that produces `ServerErrorsAsFailures`."] # [doc = ""] # [doc = " This is a convenience function that simply calls `SharedClassifier::new`."] pub fn make_classifier () -> SharedClassifier < Self > { SharedClassifier :: new (Self :: new ()) } }
};
}
