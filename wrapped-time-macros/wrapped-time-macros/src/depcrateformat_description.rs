// Generated macro for format_description (function)
macro_rules! Depcrateformat_description {
() => {
// Module: crate
// Provides: {"format_description"}
// Dependencies: {}
# [cfg (any (feature = "formatting" , feature = "parsing"))] # [proc_macro] pub fn format_description (input : TokenStream) -> TokenStream { (| | { let mut input = input . into_iter () . peekable () ; let version = parse_format_description_version :: < false > (& mut input) ? ; let (span , string) = helpers :: get_string_literal (input) ? ; let items = format_description :: parse_with_version (version , & string , span) ? ; Ok (quote_ ! { const { use :: time :: format_description :: { *, modifier ::* } ; & [# S (items . into_iter () . map (| item | quote_ ! { # S (item) , }) . collect ::< TokenStream > ())] as & [BorrowedFormatItem] } }) }) () . unwrap_or_else (| err : Error | err . to_compile_error ()) }
};
}
