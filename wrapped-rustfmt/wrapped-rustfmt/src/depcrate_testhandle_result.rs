// Generated macro for handle_result (function)
macro_rules! Depcrate_testhandle_result {
() => {
// Module: crate::test
// Provides: {"handle_result"}
// Dependencies: {}
fn handle_result (result : HashMap < PathBuf , String > , target : Option < & str > ,) -> Result < () , IdempotentCheckError > { let mut failures = HashMap :: new () ; for (file_name , fmt_text) in result { let target = get_target (& file_name , target) ; let open_error = format ! ("couldn't open target {target:?}") ; let mut f = fs :: File :: open (& target) . expect (& open_error) ; let mut text = String :: new () ; let read_error = format ! ("failed reading target {target:?}") ; f . read_to_string (& mut text) . expect (& read_error) ; if ! string_eq_ignore_newline_repr (& fmt_text , & text) { if std :: env :: var_os ("RUSTC_BLESS") . is_some_and (| v | v != "0") { std :: fs :: write (target , fmt_text) . unwrap () ; continue ; } let diff = make_diff (& text , & fmt_text , DIFF_CONTEXT_SIZE) ; assert ! (! diff . is_empty () , "Empty diff? Maybe due to a missing a newline at the end of a file?") ; failures . insert (file_name , diff) ; } } if failures . is_empty () { Ok (()) } else { Err (IdempotentCheckError :: Mismatch (failures)) } }
};
}
