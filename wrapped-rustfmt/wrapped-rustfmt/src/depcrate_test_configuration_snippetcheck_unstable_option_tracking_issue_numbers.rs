// Generated macro for check_unstable_option_tracking_issue_numbers (function)
macro_rules! Depcrate_test_configuration_snippetcheck_unstable_option_tracking_issue_numbers {
() => {
// Module: crate::test::configuration_snippet
// Provides: {"check_unstable_option_tracking_issue_numbers"}
// Dependencies: {}
# [test] fn check_unstable_option_tracking_issue_numbers () { let tracking_issue = regex :: Regex :: new (r"\(tracking issue: \[#(?P<number>\d+)\]\((?P<link>\S+)\)\)") . expect ("failed creating configuration pattern") ; let lines = BufReader :: new (fs :: File :: open (Path :: new (CONFIGURATIONS_FILE_NAME)) . unwrap_or_else (| _ | panic ! ("couldn't read file {}" , CONFIGURATIONS_FILE_NAME)) ,) . lines () . map (Result :: unwrap) . enumerate () ; for (idx , line) in lines { if let Some (capture) = tracking_issue . captures (& line) { let number = capture . name ("number") . unwrap () . as_str () ; let link = capture . name ("link") . unwrap () . as_str () ; assert ! (link . ends_with (number) , "{} on line {} does not point to issue #{}" , link , idx + 1 , number ,) ; } } }
};
}
