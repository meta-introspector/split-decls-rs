// Generated macro for ServeDir (struct)
macro_rules! Depcrate_services_fs_serve_dirServeDir {
() => {
// Module: crate::services::fs::serve_dir
// Provides: {"ServeDir"}
// Dependencies: {}
# [doc = " Service that serves files from a given directory and all its sub directories."] # [doc = ""] # [doc = " The `Content-Type` will be guessed from the file extension."] # [doc = ""] # [doc = " An empty response with status `404 Not Found` will be returned if:"] # [doc = ""] # [doc = " - The file doesn't exist"] # [doc = " - Any segment of the path contains `..`"] # [doc = " - Any segment of the path contains a backslash"] # [doc = " - On unix, any segment of the path referenced as directory is actually an"] # [doc = "   existing file (`/file.html/something`)"] # [doc = " - We don't have necessary permissions to read the file"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use tower_http::services::ServeDir;"] # [doc = ""] # [doc = " // This will serve files in the \"assets\" directory and"] # [doc = " // its subdirectories"] # [doc = " let service = ServeDir::new(\"assets\");"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct ServeDir < F = DefaultServeDirFallback > { base : PathBuf , buf_chunk_size : usize , precompressed_variants : Option < PrecompressedVariants > , variant : ServeVariant , fallback : Option < F > , call_fallback_on_method_not_allowed : bool , }
};
}
