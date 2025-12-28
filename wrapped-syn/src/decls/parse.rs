macro_rules! parse {
    () => {
        # [doc = " Parse tokens of source code into the chosen syntax tree node."] # [doc = ""] # [doc = " This is preferred over parsing a string because tokens are able to preserve"] # [doc = " information about where in the user's code they were originally written (the"] # [doc = " \"span\" of the token), possibly allowing the compiler to produce better error"] # [doc = " messages."] # [doc = ""] # [doc = " This function parses a `proc_macro::TokenStream` which is the type used for"] # [doc = " interop with the compiler in a procedural macro. To parse a"] # [doc = " `proc_macro2::TokenStream`, use [`syn::parse2`] instead."] # [doc = ""] # [doc = " [`syn::parse2`]: parse2"] # [doc = ""] # [doc = " This function enforces that the input is fully parsed. If there are any"] # [doc = " unparsed tokens at the end of the stream, an error is returned."] # [cfg (all (feature = "parsing" , feature = "proc-macro"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "parsing" , feature = "proc-macro"))))] pub fn parse < T : parse :: Parse > (tokens : proc_macro :: TokenStream) -> Result < T > { parse :: Parser :: parse (T :: parse , tokens) }
    };
}

parse!()