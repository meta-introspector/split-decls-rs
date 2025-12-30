// Generated macro for is_hidden (function)
macro_rules! Depcrate_fileis_hidden {
() => {
// Module: crate::file
// Provides: {"is_hidden"}
// Dependencies: {}
# [doc = " Returns true if and only if the given file attributes contain the"] # [doc = " `FILE_ATTRIBUTE_HIDDEN` attribute."] pub fn is_hidden (file_attributes : u64) -> bool { file_attributes & (FILE_ATTRIBUTE_HIDDEN as u64) > 0 }
};
}
