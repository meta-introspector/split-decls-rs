// Generated macro for load_pem_certs_from_dir (function)
macro_rules! Depcrateload_pem_certs_from_dir {
() => {
// Module: crate
// Provides: {"load_pem_certs_from_dir"}
// Dependencies: {}
# [doc = " Load certificate from certificate directory (what OpenSSL calls CAdir)"] fn load_pem_certs_from_dir (dir : & Path , out : & mut CertificateResult) { let dir_reader = match fs :: read_dir (dir) { Ok (reader) => reader , Err (err) => { out . io_error (err , dir , "opening directory") ; return ; } } ; for entry in dir_reader { let entry = match entry { Ok (entry) => entry , Err (err) => { out . io_error (err , dir , "reading directory entries") ; continue ; } } ; let path = entry . path () ; let metadata = match fs :: metadata (& path) { Ok (metadata) => metadata , Err (e) if e . kind () == io :: ErrorKind :: NotFound => { continue ; } Err (e) => { out . io_error (e , & path , "failed to open file") ; continue ; } } ; if metadata . is_file () { load_pem_certs (& path , out) ; } } }
};
}
