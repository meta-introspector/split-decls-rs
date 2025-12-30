// Generated macro for parsing (module)
macro_rules! Depcrate_macparsing {
() => {
// Module: crate::mac
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub mod parsing { use super :: * ; use proc_macro2 :: { TokenNode , TokenTree } ; use synom :: tokens :: * ; use synom :: { Synom , PResult , Cursor , parse_error } ; impl Synom for Mac { named ! (parse -> Self , do_parse ! (what : syn ! (Path) >> bang : syn ! (Bang) >> body : call ! (:: TokenTree :: parse_delimited) >> (Mac { path : what , bang_token : bang , ident : None , tokens : vec ! [body] , }))) ; } impl :: TokenTree { pub fn parse_list (input : Cursor) -> PResult < Vec < Self > > { Ok ((Cursor :: empty () , input . token_stream () . into_iter () . map (:: TokenTree) . collect ())) } pub fn parse_delimited (input : Cursor) -> PResult < Self > { match input . token_tree () { Some ((rest , token @ TokenTree { kind : TokenNode :: Group (..) , .. })) => { Ok ((rest , :: TokenTree (token))) } _ => parse_error () , } } } }
};
}
