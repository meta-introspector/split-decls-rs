// Generated macro for load_certs_from_paths (function)
macro_rules! Depcrateload_certs_from_paths {
() => {
// Module: crate
// Provides: {"load_certs_from_paths"}
// Dependencies: {}
# [doc = " Load certificates from the given paths."] # [doc = ""] # [doc = " If both are `None`, returns an empty [`CertificateResult`]."] # [doc = ""] # [doc = " If `file` is `Some`, it is always used, so it must be a path to an existing,"] # [doc = " accessible file from which certificates can be loaded successfully. While parsing,"] # [doc = " the rustls-pki-types PEM parser will ignore parts of the file which are"] # [doc = " not considered part of a certificate. Certificates which are not in the right"] # [doc = " format (PEM) or are otherwise corrupted may get ignored silently."] # [doc = ""] # [doc = " If `dir` is defined, a directory must exist at this path, and all files"] # [doc = " contained in it must be loaded successfully, subject to the rules outlined above for `file`."] # [doc = " The directory is not scanned recursively and may be empty."] pub fn load_certs_from_paths (file : Option < & Path > , dir : Option < & Path >) -> CertificateResult { let dir = match dir { Some (d) => vec ! [d] , None => Vec :: new () , } ; load_certs_from_paths_internal (file , dir . as_ref ()) }
};
}
