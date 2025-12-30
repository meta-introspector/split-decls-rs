// Generated macro for test (module)
macro_rules! Depcrate_os_macos_itemtest {
() => {
// Module: crate::os::macos::item
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: item :: * ; use crate :: os :: macos :: certificate :: SecCertificateExt ; use crate :: os :: macos :: test :: keychain ; use tempfile :: tempdir ; # [test] fn find_certificate () { let dir = p ! (tempdir ()) ; let keychain = keychain (dir . path ()) ; let results = p ! (ItemSearchOptions :: new () . keychains (& [keychain]) . class (ItemClass :: certificate ()) . search ()) ; assert_eq ! (1 , results . len ()) ; let SearchResult :: Ref (Reference :: Certificate (certificate)) = & results [0] else { panic ! ("expected certificate") } ; assert_eq ! ("foobar.com" , p ! (certificate . common_name ())) ; } }
};
}
