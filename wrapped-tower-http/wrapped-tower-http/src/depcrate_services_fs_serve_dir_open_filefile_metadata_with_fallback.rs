// Generated macro for file_metadata_with_fallback (function)
macro_rules! Depcrate_services_fs_serve_dir_open_filefile_metadata_with_fallback {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"file_metadata_with_fallback"}
// Dependencies: {}
async fn file_metadata_with_fallback (mut path : PathBuf , mut negotiated_encoding : Vec < (Encoding , QValue) > ,) -> io :: Result < (Metadata , Option < Encoding >) > { let (file , encoding) = loop { let encoding = preferred_encoding (& mut path , & negotiated_encoding) ; match (tokio :: fs :: metadata (& path) . await , encoding) { (Ok (file) , maybe_encoding) => break (file , maybe_encoding) , (Err (err) , Some (encoding)) if err . kind () == io :: ErrorKind :: NotFound => { path . set_extension (OsStr :: new ("")) ; negotiated_encoding . retain (| (negotiated_encoding , _) | * negotiated_encoding != encoding) ; } (Err (err) , _) => return Err (err) , } } ; Ok ((file , encoding)) }
};
}
