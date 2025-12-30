// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_parse_parserimpl_1113 {
() => {
// Module: crate::parse::parser
// Provides: {"impl_1113"}
// Dependencies: {}
impl < 'a > ParserBuilder < 'a > { pub (crate) fn input (mut self , input : Input) -> ParserBuilder < 'a > { self . input = Some (input) ; self } pub (crate) fn psess (mut self , psess : & 'a ParseSess) -> ParserBuilder < 'a > { self . psess = Some (psess) ; self } pub (crate) fn build (self) -> Result < Parser < 'a > , ParserError > { let psess = self . psess . ok_or (ParserError :: NoParseSess) ? ; let input = self . input . ok_or (ParserError :: NoInput) ? ; let parser = match Self :: parser (psess . inner () , input) { Ok (p) => p , Err (diagnostics) => { psess . emit_diagnostics (diagnostics) ; return Err (ParserError :: ParserCreationError) ; } } ; Ok (Parser { parser }) } fn parser (psess : & 'a rustc_session :: parse :: ParseSess , input : Input ,) -> Result < RawParser < 'a > , Vec < Diag < 'a > > > { match input { Input :: File (ref file) => new_parser_from_file (psess , file , None) , Input :: Text (text) => new_parser_from_source_str (psess , rustc_span :: FileName :: Custom ("stdin" . to_owned ()) , text ,) , } } }
};
}
