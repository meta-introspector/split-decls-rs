// Generated macro for test (module)
macro_rules! Depcrate_os_macos_keychaintest {
() => {
// Module: crate::os::macos::keychain
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use tempfile :: tempdir ; use super :: * ; # [test] fn create_options () { let dir = tempdir () . unwrap () ; let mut keychain = CreateOptions :: new () . password ("foobar") . create (dir . path () . join ("test.keychain")) . unwrap () ; keychain . set_settings (& KeychainSettings :: new ()) . unwrap () ; } # [test] fn disable_user_interaction () { assert ! (SecKeychain :: user_interaction_allowed () . unwrap ()) ; { let _lock = SecKeychain :: disable_user_interaction () . unwrap () ; assert ! (! SecKeychain :: user_interaction_allowed () . unwrap ()) ; } assert ! (SecKeychain :: user_interaction_allowed () . unwrap ()) ; } }
};
}
