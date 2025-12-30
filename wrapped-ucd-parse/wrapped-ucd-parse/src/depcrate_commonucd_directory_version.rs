// Generated macro for ucd_directory_version (function)
macro_rules! Depcrate_commonucd_directory_version {
() => {
// Module: crate::common
// Provides: {"ucd_directory_version"}
// Dependencies: {}
# [doc = " Given a path pointing at the root of the `ucd_dir`, attempts to determine"] # [doc = " it's unicode version."] # [doc = ""] # [doc = " This just checks the readme and the very first line of PropList.txt -- in"] # [doc = " practice this works for all versions of UCD since 4.1.0."] pub fn ucd_directory_version < D : ? Sized + AsRef < Path > > (ucd_dir : & D ,) -> Result < (u64 , u64 , u64) , Error > { fn ucd_directory_version_inner (ucd_dir : & Path ,) -> Result < (u64 , u64 , u64) , Error > { let re_version_rx = regex ! (r"-([0-9]+).([0-9]+).([0-9]+).txt") ; let proplist = ucd_dir . join ("PropList.txt") ; let contents = first_line (& proplist) ? ; let caps = match re_version_rx . captures (& contents) { Some (c) => c , None => { return err ! ("Failed to find version in line {:?}" , contents) } } ; let capture_to_num = | n | { caps . get (n) . unwrap () . as_str () . parse :: < u64 > () . map_err (| e | Error { kind : ErrorKind :: Parse (format ! ("Failed to parse version from {:?} in PropList.txt: {}" , contents , e)) , line : Some (0) , path : Some (proplist . clone ()) , }) } ; let major = capture_to_num (1) ? ; let minor = capture_to_num (2) ? ; let patch = capture_to_num (3) ? ; Ok ((major , minor , patch)) } ucd_directory_version_inner (ucd_dir . as_ref ()) }
};
}
