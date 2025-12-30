// Generated macro for read_dir (function)
macro_rules! Depcrate_utilread_dir {
() => {
// Module: crate::util
// Provides: {"read_dir"}
// Dependencies: {}
# [doc = " Similar to std::fs::read_dir except it returns a sorted Vec,"] # [doc = " which is important to make the code generation deterministic."] pub (crate) fn read_dir < P > (path : P) -> std :: io :: Result < Vec < PathBuf > > where P : AsRef < Path > , { let mut entries = fs :: read_dir (path) ? . map (| entry | Ok (entry ? . path ())) . collect :: < std :: io :: Result < Vec < _ > > > () ? ; entries . sort () ; Ok (entries) }
};
}
