// Generated macro for wikidata_impl (function)
macro_rules! Depcrate_macros_wikidatawikidata_impl {
() => {
// Module: crate::macros::wikidata
// Provides: {"wikidata_impl"}
// Dependencies: {}
# [decl (fn , name = "wikidata_impl" , vis = "pub" , hash = "9f83104c")] pub fn wikidata_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🗃️ WIKIDATA! Conceptual interaction: \"{}\"\n" , # description) ; () } . into () }
};
}
