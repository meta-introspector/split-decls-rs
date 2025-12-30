// Generated macro for error (function)
macro_rules! Depcrate_resulterror {
() => {
// Module: crate::result
// Provides: {"error"}
// Dependencies: {}
# [doc = "\nA streaming result with a generic failure.\n\nMore detailed diagnostic information will need to be stored elsewhere.\n"] # [inline (always)] pub fn error < T > () -> crate :: Result < T > { Err (Error :: new ()) }
};
}
