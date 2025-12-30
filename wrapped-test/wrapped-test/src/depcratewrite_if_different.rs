// Generated macro for write_if_different (function)
macro_rules! Depcratewrite_if_different {
() => {
// Module: crate
// Provides: {"write_if_different"}
// Dependencies: {}
# [doc = " Returns `true` if the file was written, or `false` if the file is the same"] # [doc = " as it was already on disk."] fn write_if_different (path : & Path , contents : impl AsRef < [u8] >) -> Result < bool > { let contents = contents . as_ref () ; if let Ok (prev) = fs :: read (path) { if prev == contents { return Ok (false) ; } } if let Some (parent) = path . parent () { fs :: create_dir_all (parent) . with_context (| | format ! ("failed to create directory {parent:?}")) ? ; } fs :: write (path , contents) . with_context (| | format ! ("failed to write {path:?}")) ? ; Ok (true) }
};
}
