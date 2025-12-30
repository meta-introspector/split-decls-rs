// Generated macro for impl_1059 (impl)
macro_rules! Depcrate_classifyimpl_1059 {
() => {
// Module: crate::classify
// Provides: {"impl_1059"}
// Dependencies: {}
impl < C > MakeClassifier for SharedClassifier < C > where C : ClassifyResponse + Clone , { type FailureClass = C :: FailureClass ; type ClassifyEos = C :: ClassifyEos ; type Classifier = C ; fn make_classifier < B > (& self , _req : & Request < B >) -> Self :: Classifier { self . classifier . clone () } }
};
}
