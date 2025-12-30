// Generated macro for bind_getprocaddr_make_credential (function)
macro_rules! Depcratebind_getprocaddr_make_credential {
() => {
// Module: crate
// Provides: {"bind_getprocaddr_make_credential"}
// Dependencies: {}
# [allow (unused)] pub fn bind_getprocaddr_make_credential () -> Option < WebAuthNAuthenticatorMakeCredential > { unsafe { delay_load (windows_strings :: s ! ("webauthn.dll") . as_ptr () , windows_strings :: s ! ("WebAuthNAuthenticatorMakeCredential") . as_ptr () ,) } }
};
}
