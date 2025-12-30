// Generated macro for EncryptedClientHelloError (enum)
macro_rules! Depcrate_errorEncryptedClientHelloError {
() => {
// Module: crate::error
// Provides: {"EncryptedClientHelloError"}
// Dependencies: {}
# [doc = " An error that occurred while handling Encrypted Client Hello (ECH)."] # [non_exhaustive] # [derive (Debug , Clone , Eq , PartialEq)] pub enum EncryptedClientHelloError { # [doc = " The provided ECH configuration list was invalid."] InvalidConfigList , # [doc = " No compatible ECH configuration."] NoCompatibleConfig , # [doc = " The client configuration has server name indication (SNI) disabled."] SniRequired , }
};
}
