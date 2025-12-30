// Generated macro for check_section (function)
macro_rules! Depcrate_alphabeticalcheck_section {
() => {
// Module: crate::alphabetical
// Provides: {"check_section"}
// Dependencies: {}
fn check_section < 'a > (file : impl Display , lines : impl Iterator < Item = (usize , & 'a str) > , err : & mut dyn FnMut (& str) -> std :: io :: Result < () > , bad : & mut bool ,) { let mut prev_line = String :: new () ; let mut first_indent = None ; let mut in_split_line = None ; for (idx , line) in lines { if line . is_empty () { continue ; } if line . contains (START_MARKER) { tidy_error_ext ! (err , bad , "{file}:{} found `{START_MARKER}` expecting `{END_MARKER}`" , idx + 1) ; return ; } if line . contains (END_MARKER) { return ; } let indent = first_indent . unwrap_or_else (| | { let indent = indentation (line) ; first_indent = Some (indent) ; indent }) ; let line = if let Some (prev_split_line) = in_split_line { in_split_line = None ; format ! ("{prev_split_line}{}" , line . trim_start ()) } else { line . to_string () } ; if indentation (& line) != indent { continue ; } let trimmed_line = line . trim_start_matches (' ') ; if trimmed_line . starts_with ("//") || (trimmed_line . starts_with ('#') && ! trimmed_line . starts_with ("#!")) || trimmed_line . starts_with (is_close_bracket) { continue ; } if line . trim_end () . ends_with ('(') { in_split_line = Some (line) ; continue ; } let prev_line_trimmed_lowercase = prev_line . trim_start_matches (' ') ; if version_sort (trimmed_line , prev_line_trimmed_lowercase) . is_lt () { tidy_error_ext ! (err , bad , "{file}:{}: line not in alphabetical order" , idx + 1) ; } prev_line = line ; } tidy_error_ext ! (err , bad , "{file}: reached end of file expecting `{END_MARKER}`") }
};
}
