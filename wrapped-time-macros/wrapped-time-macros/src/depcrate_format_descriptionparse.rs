// Generated macro for parse (function)
macro_rules! Depcrate_format_descriptionparse {
() => {
// Module: crate::format_description
// Provides: {"parse"}
// Dependencies: {}
fn parse < const VERSION : u8 > (s : & [u8] , proc_span : proc_macro :: Span ,) -> Result < Vec < public :: OwnedFormatItem > , crate :: Error > { let mut lexed = lexer :: lex :: < VERSION > (s , proc_span) ; let ast = ast :: parse :: < _ , VERSION > (& mut lexed) ; let format_items = format_item :: parse (ast) ; Ok (format_items . map (| res | res . map (Into :: into)) . collect :: < Result < _ , _ > > () ?) }
};
}
