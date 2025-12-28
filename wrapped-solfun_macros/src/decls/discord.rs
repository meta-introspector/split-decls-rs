macro_rules! discord {
    () => {
        # [proc_macro] # [decl (fn , name = "discord" , vis = "pub" , hash = "c4bfd671")] pub fn discord (input : TokenStream) -> TokenStream { macros :: discord :: discord_impl (input) }
    };
}

discord!();