// Generated macro for check_period (function)
macro_rules! Depcrate_fluent_periodcheck_period {
() => {
// Module: crate::fluent_period
// Provides: {"check_period"}
// Dependencies: {}
fn check_period (filename : & str , contents : & str , bad : & mut bool) { if filename . contains ("codegen") { return ; } let (Ok (parse) | Err ((parse , _))) = fluent_syntax :: parser :: parse (contents) ; for entry in & parse . body { if let Entry :: Message (m) = entry { if ALLOWLIST . contains (& m . id . name) { continue ; } if let Some (pat) = & m . value && let Some (PatternElement :: TextElement { value }) = pat . elements . last () { if value . ends_with (".") && ! value . ends_with ("...") { let ll = find_line (contents , value) ; let name = m . id . name ; tidy_error ! (bad , "{filename}:{ll}: message `{name}` ends in a period") ; } } for attr in & m . attributes { if attr . id . name == "teach_note" { continue ; } if let Some (PatternElement :: TextElement { value }) = attr . value . elements . last () && value . ends_with (".") && ! value . ends_with ("...") { let ll = find_line (contents , value) ; let name = attr . id . name ; tidy_error ! (bad , "{filename}:{ll}: attr `{name}` ends in a period") ; } } } } }
};
}
