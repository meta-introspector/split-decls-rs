// Generated macro for provider_with_one_suite (function)
macro_rules! Depcrateprovider_with_one_suite {
() => {
// Module: crate
// Provides: {"provider_with_one_suite"}
// Dependencies: {}
pub fn provider_with_one_suite (provider : & CryptoProvider , suite : SupportedCipherSuite ,) -> CryptoProvider { provider_with_suites (provider , & [suite]) }
};
}
