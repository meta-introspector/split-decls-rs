// Generated macro for check_lines (function)
macro_rules! Depcrate_alphabeticalcheck_lines {
() => {
// Module: crate::alphabetical
// Provides: {"check_lines"}
// Dependencies: {}
fn check_lines < 'a > (file : & impl Display , mut lines : impl Iterator < Item = (usize , & 'a str) > , err : & mut dyn FnMut (& str) -> std :: io :: Result < () > , bad : & mut bool ,) { while let Some ((idx , line)) = lines . next () { if line . contains (END_MARKER) { tidy_error_ext ! (err , bad , "{file}:{} found `{END_MARKER}` expecting `{START_MARKER}`" , idx + 1) } if line . contains (START_MARKER) { check_section (file , & mut lines , err , bad) ; } } }
};
}
