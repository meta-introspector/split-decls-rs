// Generated macro for test_single_snapshot (function)
macro_rules! Depcrate_snapshottest_single_snapshot {
() => {
// Module: crate::snapshot
// Provides: {"test_single_snapshot"}
// Dependencies: {}
fn test_single_snapshot (name : & str , features : & str , overwrite : bool) -> anyhow :: Result < () > { println ! ("{}" , name . bold ()) ; let is_test = name . contains ("test") ; let mut args = match is_test { true => vec ! ["-q" , "tt" , name] , false => vec ! ["-q" , "rb" , name] , } ; if ! features . is_empty () { args . extend_from_slice (& ["--features" , features]) ; } let actual = run_capturing_stdout (Command :: new ("cargo") . args (& args) . env ("DEFMT_LOG" , "trace") . current_dir (SNAPSHOT_TESTS_DIRECTORY) ,) . with_context (| | name . to_string ()) ? ; if overwrite { overwrite_expected_output (name , actual . as_bytes () , is_test) ? ; return Ok (()) ; } let expected = load_expected_output (name , is_test) ? ; let diff = TextDiff :: from_lines (& expected , & actual) ; let mut actual_matches_expected = true ; for op in diff . ops () { for change in diff . iter_changes (op) { let styled_change = match change . tag () { ChangeTag :: Delete => Some (("-" . bold () . red () , change . to_string () . red ())) , ChangeTag :: Insert => Some (("+" . bold () . green () , change . to_string () . green ())) , ChangeTag :: Equal => None , } ; if let Some ((sign , change)) = styled_change { actual_matches_expected = false ; eprint ! ("{sign}{change}") ; } } } if actual_matches_expected { Ok (()) } else { Err (anyhow ! ("{}" , name)) } }
};
}
