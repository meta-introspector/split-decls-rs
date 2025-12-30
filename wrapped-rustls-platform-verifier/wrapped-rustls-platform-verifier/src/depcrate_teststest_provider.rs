// Generated macro for test_provider (function)
macro_rules! Depcrate_teststest_provider {
() => {
// Module: crate::tests
// Provides: {"test_provider"}
// Dependencies: {}
fn test_provider () -> Arc < CryptoProvider > { Arc :: new (rustls :: crypto :: ring :: default_provider ()) }
};
}
