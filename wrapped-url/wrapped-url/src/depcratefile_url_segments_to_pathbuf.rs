// Generated macro for file_url_segments_to_pathbuf (function)
macro_rules! Depcratefile_url_segments_to_pathbuf {
() => {
// Module: crate
// Provides: {"file_url_segments_to_pathbuf"}
// Dependencies: {}
# [cfg (all (feature = "std" , windows))] fn file_url_segments_to_pathbuf (estimated_capacity : usize , host : Option < & str > , segments : str :: Split < char > ,) -> Result < PathBuf , () > { file_url_segments_to_pathbuf_windows (estimated_capacity , host , segments) }
};
}
