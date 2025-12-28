macro_rules! discord_impl {
    () => {
        # [decl (fn , name = "discord_impl" , vis = "pub" , hash = "71566332")] pub fn discord_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🗣️ DISCORD! Conceptual interaction: \"{description}\"\n") ; () } . into () }
    };
}

discord_impl!()