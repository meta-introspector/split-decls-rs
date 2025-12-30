// Generated macro for check (function)
macro_rules! Depcrate_filenamescheck {
() => {
// Module: crate::filenames
// Provides: {"check"}
// Dependencies: {}
pub fn check (root_path : & Path , bad : & mut bool) { let stat_output = Command :: new ("git") . arg ("-C") . arg (root_path) . args (["ls-files" , "-z"]) . output () . unwrap () . stdout ; for filename in stat_output . split (| & b | b == 0) { match str :: from_utf8 (filename) { Err (_) => tidy_error ! (bad , r#"non-UTF8 file names are not supported: "{}""# , String :: from_utf8_lossy (filename) ,) , Ok (name) if name . chars () . any (| c | c . is_control ()) => tidy_error ! (bad , r#"control characters are not supported in file names: "{}""# , String :: from_utf8_lossy (filename) ,) , Ok (name) if name . contains (':') => tidy_error ! (bad , r#"":" is not supported in file names because of Windows compatibility: "{name}""# ,) , _ => () , } } }
};
}
