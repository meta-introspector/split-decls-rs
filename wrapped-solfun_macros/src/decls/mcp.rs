macro_rules! mcp {
    () => {
        # [proc_macro] # [decl (fn , name = "mcp" , vis = "pub" , hash = "b9e0e3db")] pub fn mcp (input : TokenStream) -> TokenStream { macros :: mcp :: mcp_impl (input) }
    };
}

mcp!()