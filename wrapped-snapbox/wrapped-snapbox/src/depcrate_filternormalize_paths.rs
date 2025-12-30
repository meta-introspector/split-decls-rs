// Generated macro for normalize_paths (function)
macro_rules! Depcrate_filternormalize_paths {
() => {
// Module: crate::filter
// Provides: {"normalize_paths"}
// Dependencies: {}
# [doc = " Normalize path separators"] # [doc = ""] # [doc = " [`std::path::MAIN_SEPARATOR`] can vary by platform, so make it consistent"] # [doc = ""] # [doc = " Note: this cannot distinguish between when a character is being used as a path separator or not"] # [doc = " and can \"normalize\" unrelated data"] pub fn normalize_paths (data : & str) -> String { normalize_paths_chars (data . chars ()) . collect () }
};
}
