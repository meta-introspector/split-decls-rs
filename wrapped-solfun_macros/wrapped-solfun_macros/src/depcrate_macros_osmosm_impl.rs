// Generated macro for osm_impl (function)
macro_rules! Depcrate_macros_osmosm_impl {
() => {
// Module: crate::macros::osm
// Provides: {"osm_impl"}
// Dependencies: {}
# [decl (fn , name = "osm_impl" , vis = "pub" , hash = "f3546a82")] pub fn osm_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🗺️ OPENSTREETMAP! Conceptual interaction: \" {{}}\"\n" , # description) ; () } . into () }
};
}
