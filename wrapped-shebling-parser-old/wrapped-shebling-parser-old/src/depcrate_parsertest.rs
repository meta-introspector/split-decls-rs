// Generated macro for test (function)
macro_rules! Depcrate_parsertest {
() => {
// Module: crate::parser
// Provides: {"test"}
// Dependencies: {}
# [allow (unused_variables , unused_imports)] pub (crate) fn test (file_path : impl AsRef < str > , source_code : & str) { use crate :: source_to_span ; use miette :: Report ; use std :: sync :: Arc ; match term (source_to_span (source_code)) . finish () { Ok ((span , res)) => { println ! ("OK SPAN {:#?}" , span) ; println ! ("OK RES {:#?}" , res) ; let diags : Vec < ParseDiagnostic > = span . extra . take_diags () ; let source_code = Arc :: new (miette :: NamedSource :: new (file_path , source_code . to_owned () + "\n" ,)) ; for diag in diags { println ! ("{:?}" , Report :: new (diag) . with_source_code (Arc :: clone (& source_code))) ; } } Err (err) => { println ! ("ERR {:#?}" , err) ; let diags = err . diags () ; let source_code = Arc :: new (miette :: NamedSource :: new (file_path , source_code . to_owned () + "\n" ,)) ; for diag in diags { println ! ("{:?}" , Report :: new (diag . clone ()) . with_source_code (Arc :: clone (& source_code))) ; } println ! ("{:?}" , Report :: new (err) . with_source_code (Arc :: clone (& source_code))) ; } } }
};
}
