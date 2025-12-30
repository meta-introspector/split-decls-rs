// Generated macro for test (module)
macro_rules! Depcrate_os_macostest {
() => {
// Module: crate::os::macos
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] pub (crate) mod test { use crate :: identity :: SecIdentity ; use crate :: item :: { ItemClass , ItemSearchOptions , Reference , SearchResult } ; use crate :: os :: macos :: keychain :: SecKeychain ; use std :: fs :: File ; use std :: io :: prelude :: * ; use std :: path :: Path ; # [must_use] pub (crate) fn identity (dir : & Path) -> SecIdentity { let keychain = keychain (dir) ; let mut items = p ! (ItemSearchOptions :: new () . class (ItemClass :: identity ()) . keychains (& [keychain]) . search ()) ; match items . pop () . unwrap () { SearchResult :: Ref (Reference :: Identity (identity)) => identity , _ => panic ! ("expected identity") , } } # [must_use] pub (crate) fn keychain (dir : & Path) -> SecKeychain { let path = dir . join ("server.keychain") ; let mut file = p ! (File :: create (& path)) ; p ! (file . write_all (include_bytes ! ("../../../test/server.keychain"))) ; drop (file) ; let mut keychain = p ! (SecKeychain :: open (& path)) ; p ! (keychain . unlock (Some ("password123"))) ; keychain } }
};
}
