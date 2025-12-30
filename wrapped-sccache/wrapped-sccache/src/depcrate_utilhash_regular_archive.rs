// Generated macro for hash_regular_archive (function)
macro_rules! Depcrate_utilhash_regular_archive {
() => {
// Module: crate::util
// Provides: {"hash_regular_archive"}
// Dependencies: {}
fn hash_regular_archive (m : & mut Digest , data : & [u8]) -> Result < () > { let archive = ArchiveFile :: parse (data) ? ; for entry in archive . members () { let entry = entry ? ; m . update (entry . name ()) ; m . update (entry . data (data) ?) ; } Ok (()) }
};
}
