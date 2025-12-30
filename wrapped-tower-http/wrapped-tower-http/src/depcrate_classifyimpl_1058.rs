// Generated macro for impl_1058 (impl)
macro_rules! Depcrate_classifyimpl_1058 {
() => {
// Module: crate::classify
// Provides: {"impl_1058"}
// Dependencies: {}
impl < C > SharedClassifier < C > { # [doc = " Create a new `SharedClassifier` from the given classifier."] pub fn new (classifier : C) -> Self where C : ClassifyResponse + Clone , { Self { classifier } } }
};
}
