macro_rules! foaf_impl {
    () => {
        # [decl (fn , name = "foaf_impl" , vis = "pub" , hash = "b71ebb67")] pub fn foaf_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🧑‍🤝‍🧑 FOAF! Conceptual interaction: \"{{}}\"\n" , # description) ; () } . into () }
    };
}

foaf_impl!();