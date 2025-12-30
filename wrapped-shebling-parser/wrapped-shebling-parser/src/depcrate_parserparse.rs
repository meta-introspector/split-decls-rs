// Generated macro for parse (function)
macro_rules! Depcrate_parserparse {
() => {
// Module: crate::parser
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (source_code : & str , file_path : & str) { use miette :: Report ; use std :: sync :: Arc ; let diags = ParseDiags :: new () ; let res = trivia (ParseSpan :: new (source_code , & diags)) . finish () ; let source_code = Arc :: new (miette :: NamedSource :: new (file_path , source_code . to_owned () + "\n" ,)) ; match res { Ok ((span , res)) => { println ! ("OK SPAN {:#?}" , span) ; println ! ("OK RES {:#?}" , res) ; } Err (err) => { println ! ("ERR {:#?}" , err) ; println ! ("{:?}" , Report :: new (err) . with_source_code (Arc :: clone (& source_code))) ; } } for diag in diags { println ! ("{:?}" , Report :: new (diag) . with_source_code (Arc :: clone (& source_code))) ; } }
};
}
