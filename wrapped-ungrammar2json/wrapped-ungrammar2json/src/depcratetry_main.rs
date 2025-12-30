// Generated macro for try_main (function)
macro_rules! Depcratetry_main {
() => {
// Module: crate
// Provides: {"try_main"}
// Dependencies: {}
fn try_main () -> io :: Result < () > { if env :: args () . count () != 1 { eprintln ! ("Usage: ungrammar2json < grammar.ungram > grammar.json") ; return Ok (()) ; } let grammar = read_stdin () ? ; let grammar = grammar . parse :: < Grammar > () . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) ? ; let mut buf = String :: new () ; grammar_to_json (& grammar , write_json :: object (& mut buf)) ; println ! ("{}" , buf) ; Ok (()) }
};
}
