macro_rules! symbols {
    () => {
        # [proc_macro] pub fn symbols (input : TokenStream) -> TokenStream { symbols :: symbols (input . into ()) . into () }
    };
}

symbols!()