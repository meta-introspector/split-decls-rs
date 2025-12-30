// Generated macro for DimacsParser (struct)
macro_rules! DepcrateDimacsParser {
() => {
// Module: crate
// Provides: {"DimacsParser"}
// Dependencies: {}
# [doc = " Parser for DIMACS CNF files."] # [doc = ""] # [doc = " This parser can consume the input in chunks while also producing the parsed result in chunks."] # [derive (Default)] pub struct DimacsParser { formula : CnfFormula , partial_clause : Vec < Lit > , header : Option < DimacsHeader > , line_number : usize , clause_count : usize , partial_lit : usize , negate_next_lit : bool , in_lit : bool , in_comment_or_header : bool , in_header : bool , start_of_line : bool , error : bool , header_line : Vec < u8 > , }
};
}
