// Generated macro for hash_key (function)
macro_rules! Depcrate_compiler_chash_key {
() => {
// Module: crate::compiler::c
// Provides: {"hash_key"}
// Dependencies: {}
# [doc = " Compute the hash key of `compiler` compiling `preprocessor_output` with `args`."] pub fn hash_key (compiler_digest : & str , language : Language , arguments : & [OsString] , extra_hashes : & [String] , env_vars : & [(OsString , OsString)] , preprocessor_output : & [u8] , plusplus : bool ,) -> String { let mut m = Digest :: new () ; m . update (compiler_digest . as_bytes ()) ; m . update (& [plusplus as u8]) ; m . update (CACHE_VERSION) ; m . update (language . as_str () . as_bytes ()) ; for arg in arguments { arg . hash (& mut HashToDigest { digest : & mut m }) ; } for hash in extra_hashes { m . update (hash . as_bytes ()) ; } for (var , val) in env_vars . iter () { if CACHED_ENV_VARS . contains (var . as_os_str ()) { var . hash (& mut HashToDigest { digest : & mut m }) ; m . update (& b"=" [..]) ; val . hash (& mut HashToDigest { digest : & mut m }) ; } } m . update (preprocessor_output) ; m . finish () }
};
}
