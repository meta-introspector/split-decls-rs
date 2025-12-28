macro_rules! service_finder {
    () => {
        # [proc_macro] # [decl (fn , name = "service_finder" , vis = "pub" , hash = "7a15222d")] pub fn service_finder (input : TokenStream) -> TokenStream { macros :: service_finder :: service_finder_impl (input) }
    };
}

service_finder!()