// Generated macro for TlsClientAuth (enum)
macro_rules! Depcrate_tlsTlsClientAuth {
() => {
// Module: crate::tls
// Provides: {"TlsClientAuth"}
// Dependencies: {}
# [doc = " Tls client authentication configuration."] pub (crate) enum TlsClientAuth { # [doc = " No client auth."] Off , # [doc = " Allow any anonymous or authenticated client."] Optional (Box < dyn Read + Send + Sync >) , # [doc = " Allow any authenticated client."] Required (Box < dyn Read + Send + Sync >) , }
};
}
