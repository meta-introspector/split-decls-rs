// Generated macro for check_error_codes_used (function)
macro_rules! Depcrate_error_codescheck_error_codes_used {
() => {
// Module: crate::error_codes
// Provides: {"check_error_codes_used"}
// Dependencies: {}
# [doc = " Stage 4: Search `compiler/` and ensure that every error code is actually used by the compiler and that no undocumented error codes exist."] fn check_error_codes_used (search_paths : & [& Path] , error_codes : & [String] , errors : & mut Vec < String > , no_longer_emitted : & [String] , verbose : bool ,) { let regex = Regex :: new (r#"\bE\d{4}\b"#) . unwrap () ; let mut found_codes = Vec :: new () ; walk_many (search_paths , | path , _is_dir | filter_dirs (path) , & mut | entry , contents | { let path = entry . path () ; if path . extension () != Some (OsStr :: new ("rs")) { return ; } for line in contents . lines () { if line . trim_start () . starts_with ("//") { continue ; } for cap in regex . captures_iter (line) { if let Some (error_code) = cap . get (0) { let error_code = error_code . as_str () . to_owned () ; if ! error_codes . contains (& error_code) { errors . push (format ! ("Error code `{error_code}` is used in the compiler but not defined and documented in `compiler/rustc_error_codes/src/lib.rs`.")) ; continue ; } found_codes . push (error_code) ; } } } }) ; for code in error_codes { if ! found_codes . contains (code) && ! no_longer_emitted . contains (code) { errors . push (format ! ("Error code `{code}` exists, but is not emitted by the compiler!\n\
                Please mark the code as no longer emitted by adding the following note to the top of the `EXXXX.md` file:\n\
                `#### Note: this error code is no longer emitted by the compiler`\n\
                Also, do not forget to mark doctests that no longer apply as `ignore (error is no longer emitted)`.")) ; } if found_codes . contains (code) && no_longer_emitted . contains (code) { verbose_print ! (verbose , "warning: Error code `{code}` is used when it's marked as \"no longer emitted\"") ; } } }
};
}
