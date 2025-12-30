// Generated macro for dollar_sgmt (function)
macro_rules! Depcrate_parser_expansiondollar_sgmt {
() => {
// Module: crate::parser::expansion
// Provides: {"dollar_sgmt"}
// Dependencies: {}
pub (super) fn dollar_sgmt (span : Span) -> ParseResult < WordSgmt > { alt ((into (alt ((dollar_exp , preceded (char ('$') , alt ((into (double_quoted) , map (single_quoted , DollarExp :: SingleQuoting) ,)) ,) ,))) , map (char ('$') , | c | WordSgmt :: Lit (c . into ())) ,)) (span) }
};
}
