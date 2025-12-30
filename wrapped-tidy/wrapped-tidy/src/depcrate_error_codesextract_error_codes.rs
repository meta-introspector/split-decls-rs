// Generated macro for extract_error_codes (function)
macro_rules! Depcrate_error_codesextract_error_codes {
() => {
// Module: crate::error_codes
// Provides: {"extract_error_codes"}
// Dependencies: {}
# [doc = " Stage 1: Parses a list of error codes from `error_codes.rs`."] fn extract_error_codes (root_path : & Path , errors : & mut Vec < String >) -> Vec < String > { let path = root_path . join (Path :: new (ERROR_CODES_PATH)) ; let file = fs :: read_to_string (& path) . unwrap_or_else (| e | panic ! ("failed to read `{path:?}`: {e}")) ; let path = path . display () ; let mut error_codes = Vec :: new () ; for (line_index , line) in file . lines () . enumerate () { let line_index = line_index + 1 ; let line = line . trim () ; if line . starts_with ('E') { let split_line = line . split_once (':') ; let Some (split_line) = split_line else { errors . push (format ! ("{path}:{line_index}: Expected a line with the format `Eabcd: abcd, \
                    but got \"{line}\" without a `:` delimiter" ,)) ; continue ; } ; let err_code = split_line . 0 . to_owned () ; if error_codes . contains (& err_code) { errors . push (format ! ("{path}:{line_index}: Found duplicate error code: `{err_code}`")) ; continue ; } let mut chars = err_code . chars () ; assert_eq ! (chars . next () , Some ('E')) ; let error_num_as_str = chars . as_str () ; let rest = split_line . 1 . split_once (',') ; let Some (rest) = rest else { errors . push (format ! ("{path}:{line_index}: Expected a line with the format `Eabcd: abcd, \
                    but got \"{line}\" without a `,` delimiter" ,)) ; continue ; } ; if error_num_as_str != rest . 0 . trim () { errors . push (format ! ("{path}:{line_index}: `{}:` should be followed by `{},` but instead found `{}` in \
                    `compiler/rustc_error_codes/src/lib.rs`" , err_code , error_num_as_str , split_line . 1 ,)) ; continue ; } if ! rest . 1 . trim () . is_empty () && ! rest . 1 . trim () . starts_with ("//") { errors . push (format ! ("{path}:{line_index}: should only have one error per line")) ; continue ; } error_codes . push (err_code) ; } } error_codes }
};
}
