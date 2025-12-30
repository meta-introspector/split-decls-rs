// Generated macro for uri (function)
macro_rules! Depcrateuri {
() => {
// Module: crate
// Provides: {"uri"}
// Dependencies: {}
# [test] fn uri () -> windows_core :: Result < () > { use b_uri :: * ; let uri = Uri :: CreateUri (windows_core :: h ! ("https://kennykerr.ca/")) ? ; assert_eq ! (uri . Domain () ?, "kennykerr.ca") ; Ok (()) }
};
}
