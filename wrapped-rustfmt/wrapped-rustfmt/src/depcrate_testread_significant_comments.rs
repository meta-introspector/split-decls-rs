// Generated macro for read_significant_comments (function)
macro_rules! Depcrate_testread_significant_comments {
() => {
// Module: crate::test
// Provides: {"read_significant_comments"}
// Dependencies: {}
fn read_significant_comments (file_name : & Path) -> HashMap < String , String > { let file = fs :: File :: open (file_name) . unwrap_or_else (| _ | panic ! ("couldn't read file {}" , file_name . display ())) ; let reader = BufReader :: new (file) ; let pattern = r"^\s*//\s*rustfmt-([^:]+):\s*(\S+)" ; let regex = regex :: Regex :: new (pattern) . expect ("failed creating pattern 1") ; let line_regex = regex :: Regex :: new (r"(^\s*$)|(^\s*//\s*rustfmt-[^:]+:\s*\S+)") . expect ("failed creating pattern 2") ; reader . lines () . map (| line | line . expect ("failed getting line")) . filter (| line | line_regex . is_match (line)) . filter_map (| line | { regex . captures_iter (& line) . next () . map (| capture | { (capture . get (1) . expect ("couldn't unwrap capture") . as_str () . to_owned () , capture . get (2) . expect ("couldn't unwrap capture") . as_str () . to_owned () ,) }) }) . collect () }
};
}
