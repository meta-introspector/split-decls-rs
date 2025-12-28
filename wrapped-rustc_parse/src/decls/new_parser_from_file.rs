macro_rules! new_parser_from_file {
    () => {
        # [doc = " Creates a new parser from a filename. On failure, the errors must be consumed via"] # [doc = " `unwrap_or_emit_fatal`, `emit`, `cancel`, etc., otherwise a panic will occur when they are"] # [doc = " dropped."] # [doc = ""] # [doc = " If a span is given, that is used on an error as the source of the problem."] pub fn new_parser_from_file < 'a > (psess : & 'a ParseSess , path : & Path , strip_tokens : StripTokens , sp : Option < Span > ,) -> Result < Parser < 'a > , Vec < Diag < 'a > > > { let sm = psess . source_map () ; let source_file = sm . load_file (path) . unwrap_or_else (| e | { let msg = format ! ("couldn't read `{}`: {}" , path . display () , e) ; let mut err = psess . dcx () . struct_fatal (msg) ; if let Ok (contents) = std :: fs :: read (path) && let Err (utf8err) = String :: from_utf8 (contents . clone ()) { utf8_error (sm , & path . display () . to_string () , sp , & mut err , utf8err . utf8_error () , & contents ,) ; } if let Some (sp) = sp { err . span (sp) ; } err . emit () ; }) ; new_parser_from_source_file (psess , source_file , strip_tokens) }
    };
}

new_parser_from_file!()