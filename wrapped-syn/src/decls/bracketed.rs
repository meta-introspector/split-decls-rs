macro_rules! deps {
    () => {
        Result!();
        ParseStream!();
        Parse!();
    };
}

macro_rules! bracketed {
    () => {
        deps!();
        # [doc = " Parse a set of square brackets and expose their content to subsequent"] # [doc = " parsers."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use quote::quote;"] # [doc = " #"] # [doc = " use proc_macro2::TokenStream;"] # [doc = " use syn::{bracketed, token, Result, Token};"] # [doc = " use syn::parse::{Parse, ParseStream};"] # [doc = ""] # [doc = " // Parse an outer attribute like:"] # [doc = " //"] # [doc = " //     #[repr(C, packed)]"] # [doc = " struct OuterAttribute {"] # [doc = "     pound_token: Token![#],"] # [doc = "     bracket_token: token::Bracket,"] # [doc = "     content: TokenStream,"] # [doc = " }"] # [doc = ""] # [doc = " impl Parse for OuterAttribute {"] # [doc = "     fn parse(input: ParseStream) -> Result<Self> {"] # [doc = "         let content;"] # [doc = "         Ok(OuterAttribute {"] # [doc = "             pound_token: input.parse()?,"] # [doc = "             bracket_token: bracketed!(content in input),"] # [doc = "             content: content.parse()?,"] # [doc = "         })"] # [doc = "     }"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     let input = quote! {"] # [doc = " #         #[repr(C, packed)]"] # [doc = " #     };"] # [doc = " #     syn::parse2::<OuterAttribute>(input).unwrap();"] # [doc = " # }"] # [doc = " ```"] # [macro_export] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] macro_rules ! bracketed { ($ content : ident in $ cursor : expr) => { match $ crate :: __private :: parse_brackets (&$ cursor) { $ crate :: __private :: Ok (brackets) => { $ content = brackets . content ; brackets . token } $ crate :: __private :: Err (error) => { return $ crate :: __private :: Err (error) ; } } } ; }
    };
}

bracketed!()