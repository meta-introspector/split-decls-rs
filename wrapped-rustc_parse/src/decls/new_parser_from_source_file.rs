macro_rules! new_parser_from_source_file {
    () => {
        # [doc = " Given a session and a `source_file`, return a parser. Returns any buffered errors from lexing"] # [doc = " the initial token stream."] fn new_parser_from_source_file (psess : & ParseSess , source_file : Arc < SourceFile > , strip_tokens : StripTokens ,) -> Result < Parser < '_ > , Vec < Diag < '_ > > > { let end_pos = source_file . end_position () ; let stream = source_file_to_stream (psess , source_file , None , strip_tokens) ? ; let mut parser = Parser :: new (psess , stream , None) ; if parser . token == token :: Eof { parser . token . span = Span :: new (end_pos , end_pos , parser . token . span . ctxt () , None) ; } Ok (parser) }
    };
}

new_parser_from_source_file!()