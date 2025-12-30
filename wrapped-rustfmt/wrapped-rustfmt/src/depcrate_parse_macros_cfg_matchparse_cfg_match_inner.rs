// Generated macro for parse_cfg_match_inner (function)
macro_rules! Depcrate_parse_macros_cfg_matchparse_cfg_match_inner {
() => {
// Module: crate::parse::macros::cfg_match
// Provides: {"parse_cfg_match_inner"}
// Dependencies: {}
fn parse_cfg_match_inner < 'a > (psess : & 'a ParseSess , mac : & 'a ast :: MacCall ,) -> Result < Vec < ast :: Item > , & 'static str > { let ts = mac . args . tokens . clone () ; let mut parser = build_stream_parser (psess . inner () , ts) ; if parser . token == TokenKind :: OpenDelim (Delimiter :: Brace) { return Err ("Expression position cfg_match! not yet supported") ; } let mut items = vec ! [] ; while parser . token . kind != TokenKind :: Eof { if ! parser . eat_keyword (exp ! (Underscore)) { parser . parse_attr_item (ForceCollect :: No) . map_err (| e | { e . cancel () ; "Failed to parse attr item" }) ? ; } if ! parser . eat (exp ! (FatArrow)) { return Err ("Expected a fat arrow") ; } if ! parser . eat (exp ! (OpenBrace)) { return Err ("Expected an opening brace") ; } while parser . token != TokenKind :: CloseDelim (Delimiter :: Brace) && parser . token . kind != TokenKind :: Eof { let item = match parser . parse_item (ForceCollect :: No) { Ok (Some (item_ptr)) => item_ptr . into_inner () , Ok (None) => continue , Err (err) => { err . cancel () ; parser . psess . dcx () . reset_err_count () ; return Err ("Expected item inside cfg_match block, but failed to parse it as an item" ,) ; } } ; if let ast :: ItemKind :: Mod (..) = item . kind { items . push (item) ; } } if ! parser . eat (exp ! (CloseBrace)) { return Err ("Expected a closing brace") ; } if parser . eat (exp ! (Eof)) { break ; } } Ok (items) }
};
}
