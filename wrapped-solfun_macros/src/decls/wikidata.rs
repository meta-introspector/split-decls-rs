macro_rules! wikidata {
    () => {
        # [proc_macro] # [decl (fn , name = "wikidata" , vis = "pub" , hash = "168c75a6")] pub fn wikidata (input : TokenStream) -> TokenStream { macros :: wikidata :: wikidata_impl (input) }
    };
}

wikidata!()