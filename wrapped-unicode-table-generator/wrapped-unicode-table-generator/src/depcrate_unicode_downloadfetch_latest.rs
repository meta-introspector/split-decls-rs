// Generated macro for fetch_latest (function)
macro_rules! Depcrate_unicode_downloadfetch_latest {
() => {
// Module: crate::unicode_download
// Provides: {"fetch_latest"}
// Dependencies: {}
pub fn fetch_latest () { let directory = Path :: new (UNICODE_DIRECTORY) ; if directory . exists () { eprintln ! ("Not refetching unicode data, already exists, please delete {directory:?} to regenerate" ,) ; return ; } if let Err (e) = std :: fs :: create_dir_all (directory) { panic ! ("Failed to create {UNICODE_DIRECTORY:?}: {e}") ; } let output = fetch (README) ; let current = std :: fs :: read_to_string (directory . join (README)) . unwrap_or_default () ; if current . as_bytes () != & output . stdout [..] { std :: fs :: write (directory . join (README) , output . stdout) . unwrap () ; } for resource in RESOURCES { let output = fetch (resource) ; std :: fs :: write (directory . join (resource) , output . stdout) . unwrap () ; } }
};
}
