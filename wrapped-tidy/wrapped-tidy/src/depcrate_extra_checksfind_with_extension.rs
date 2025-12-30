// Generated macro for find_with_extension (function)
macro_rules! Depcrate_extra_checksfind_with_extension {
() => {
// Module: crate::extra_checks
// Provides: {"find_with_extension"}
// Dependencies: {}
# [doc = " Check git for tracked files matching an extension"] fn find_with_extension (root_path : & Path , find_dir : Option < & Path > , extensions : & [& OsStr] ,) -> Result < Vec < PathBuf > , Error > { let stat_output = Command :: new ("git") . arg ("-C") . arg (root_path) . args (["status" , "--short"]) . output () ? . stdout ; if String :: from_utf8_lossy (& stat_output) . lines () . filter (| ln | ln . starts_with ('?')) . count () > 0 { eprintln ! ("found untracked files, ignoring") ; } let mut output = Vec :: new () ; let binding = { let mut command = Command :: new ("git") ; command . arg ("-C") . arg (root_path) . args (["ls-files"]) ; if let Some (find_dir) = find_dir { command . arg (find_dir) ; } command . output () ? } ; let tracked = String :: from_utf8_lossy (& binding . stdout) ; for line in tracked . lines () { let line = line . trim () ; let path = Path :: new (line) ; let Some (ref extension) = path . extension () else { continue ; } ; if extensions . contains (extension) { output . push (root_path . join (path)) ; } } Ok (output) }
};
}
