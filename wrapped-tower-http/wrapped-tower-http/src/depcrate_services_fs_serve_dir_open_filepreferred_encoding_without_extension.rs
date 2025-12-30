// Generated macro for preferred_encoding_without_extension (function)
macro_rules! Depcrate_services_fs_serve_dir_open_filepreferred_encoding_without_extension {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"preferred_encoding_without_extension"}
// Dependencies: {}
# [test] fn preferred_encoding_without_extension () { let mut path = PathBuf :: from ("hello") ; preferred_encoding (& mut path , & [(Encoding :: Gzip , QValue :: one ())]) ; assert_eq ! (path , PathBuf :: from ("hello.gz")) ; }
};
}
