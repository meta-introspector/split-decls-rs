// Generated macro for tests (module)
macro_rules! Depcrate_versioned_sanitizedtests {
() => {
// Module: crate::versioned::sanitized
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , solana_hash :: Hash , solana_message :: { v0 , VersionedMessage } , solana_pubkey :: Pubkey , } ; # [test] fn test_try_new_with_invalid_signatures () { let tx = VersionedTransaction { signatures : vec ! [] , message : VersionedMessage :: V0 (v0 :: Message :: try_compile (& Pubkey :: new_unique () , & [] , & [] , Hash :: default ()) . unwrap () ,) , } ; assert_eq ! (SanitizedVersionedTransaction :: try_new (tx) , Err (SanitizeError :: IndexOutOfBounds)) ; } # [test] fn test_try_new () { let mut message = v0 :: Message :: try_compile (& Pubkey :: new_unique () , & [] , & [] , Hash :: default ()) . unwrap () ; message . header . num_readonly_signed_accounts += 1 ; let tx = VersionedTransaction { signatures : vec ! [Signature :: default ()] , message : VersionedMessage :: V0 (message) , } ; assert_eq ! (SanitizedVersionedTransaction :: try_new (tx) , Err (SanitizeError :: InvalidValue)) ; } }
};
}
