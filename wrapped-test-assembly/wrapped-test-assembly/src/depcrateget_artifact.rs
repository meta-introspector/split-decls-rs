// Generated macro for get_artifact (function)
macro_rules! Depcrateget_artifact {
() => {
// Module: crate
// Provides: {"get_artifact"}
// Dependencies: {}
pub fn get_artifact (result_stream : & [u8] , package : & str) -> PathBuf { use cargo_metadata :: Message ; Message :: parse_stream (result_stream) . find_map (| message | { if let Message :: CompilerArtifact (artifact) = message . unwrap () { if artifact . target . name == package && artifact . filenames . len () == 2 { let path = artifact . filenames [1] . clone () ; let stem = path . file_stem () . unwrap () . strip_prefix ("lib") . unwrap () ; return Some (path . with_file_name (format ! ("{stem}.s"))) ; } } None }) . unwrap_or_else (| | { panic ! ("Could not find package data:\n{}" , String :: from_utf8_lossy (result_stream)) }) . into_std_path_buf () }
};
}
