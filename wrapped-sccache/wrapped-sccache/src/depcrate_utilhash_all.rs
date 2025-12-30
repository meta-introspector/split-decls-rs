// Generated macro for hash_all (function)
macro_rules! Depcrate_utilhash_all {
() => {
// Module: crate::util
// Provides: {"hash_all"}
// Dependencies: {}
# [doc = " Calculate the digest of each file in `files` on background threads in"] # [doc = " `pool`."] pub async fn hash_all (files : & [PathBuf] , pool : & tokio :: runtime :: Handle) -> Result < Vec < String > > { let start = time :: Instant :: now () ; let count = files . len () ; let iter = files . iter () . map (move | f | Digest :: file (f , pool)) ; let hashes = futures :: future :: try_join_all (iter) . await ? ; trace ! ("Hashed {} files in {}" , count , fmt_duration_as_secs (& start . elapsed ())) ; Ok (hashes) }
};
}
