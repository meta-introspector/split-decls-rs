macro_rules! figlet_impl {
    () => {
        # [decl (fn , name = "figlet_impl" , vis = "pub" , hash = "e0e55bdf")] pub fn figlet_impl (input : TokenStream) -> TokenStream { let text_literal = parse_macro_input ! (input as LitStr) ; let span = text_literal . span () ; let simulated_figlet_output = format ! ("FIGLET: {}" , text_literal . value ()) ; quote_spanned ! { span => eprintln ! ("\n{}\n" , # simulated_figlet_output) ; () } . into () }
    };
}

figlet_impl!()