// Generated macro for format_snippet (function)
macro_rules! Depcrateformat_snippet {
() => {
// Module: crate
// Provides: {"format_snippet"}
// Dependencies: {}
# [doc = " Format the given snippet. The snippet is expected to be *complete* code."] # [doc = " When we cannot parse the given snippet, this function returns `None`."] fn format_snippet (snippet : & str , config : & Config , is_macro_def : bool) -> Option < FormattedSnippet > { let mut config = config . clone () ; panic :: catch_unwind (| | { let mut out : Vec < u8 > = Vec :: with_capacity (snippet . len () * 2) ; config . set () . emit_mode (config :: EmitMode :: Stdout) ; config . set () . verbose (Verbosity :: Quiet) ; config . set () . show_parse_errors (false) ; if is_macro_def { config . set () . error_on_unformatted (true) ; } let (formatting_error , result) = { let input = Input :: Text (snippet . into ()) ; let mut session = Session :: new (config , Some (& mut out)) ; let result = session . format_input_inner (input , is_macro_def) ; (session . errors . has_macro_format_failure || session . out . as_ref () . unwrap () . is_empty () && ! snippet . is_empty () || result . is_err () || (is_macro_def && session . has_unformatted_code_errors ()) , result ,) } ; if formatting_error { None } else { String :: from_utf8 (out) . ok () . map (| snippet | FormattedSnippet { snippet , non_formatted_ranges : result . unwrap () . non_formatted_ranges , }) } }) . ok () ? }
};
}
