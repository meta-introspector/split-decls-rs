// Generated macro for should_skip_module (function)
macro_rules! Depcrate_formattingshould_skip_module {
() => {
// Module: crate::formatting
// Provides: {"should_skip_module"}
// Dependencies: {}
# [doc = " Determine if a module should be skipped. True if the module should be skipped, false otherwise."] fn should_skip_module < T : FormatHandler > (config : & Config , context : & FormatContext < '_ , T > , input_is_stdin : bool , main_file : & FileName , path : & FileName , module : & Module < '_ > ,) -> bool { if contains_skip (module . attrs ()) { return true ; } if config . skip_children () && path != main_file { return true ; } if ! input_is_stdin && context . ignore_file (path) { return true ; } if ! input_is_stdin && ! config . format_generated_files () { let source_file = context . psess . span_to_file_contents (module . span) ; let src = source_file . src . as_ref () . expect ("SourceFile without src") ; if is_generated_file (src , config) { return true ; } } false }
};
}
