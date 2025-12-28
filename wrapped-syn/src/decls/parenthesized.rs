macro_rules! deps {
    () => {
        Result!();
        Parse!();
        ParseStream!();
    };
}

macro_rules! parenthesized {
    () => {
        deps!();
        # [doc = " Parse a set of parentheses and expose their content to subsequent parsers."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use quote::quote;"] # [doc = " #"] # [doc = " use syn::{parenthesized, token, Ident, Result, Token, Type};"] # [doc = " use syn::parse::{Parse, ParseStream};"] # [doc = " use syn::punctuated::Punctuated;"] # [doc = ""] # [doc = " // Parse a simplified tuple struct syntax like:"] # [doc = " //"] # [doc = " //     struct S(A, B);"] # [doc = " struct TupleStruct {"] # [doc = "     struct_token: Token![struct],"] # [doc = "     ident: Ident,"] # [doc = "     paren_token: token::Paren,"] # [doc = "     fields: Punctuated<Type, Token![,]>,"] # [doc = "     semi_token: Token![;],"] # [doc = " }"] # [doc = ""] # [doc = " impl Parse for TupleStruct {"] # [doc = "     fn parse(input: ParseStream) -> Result<Self> {"] # [doc = "         let content;"] # [doc = "         Ok(TupleStruct {"] # [doc = "             struct_token: input.parse()?,"] # [doc = "             ident: input.parse()?,"] # [doc = "             paren_token: parenthesized!(content in input),"] # [doc = "             fields: content.parse_terminated(Type::parse, Token![,])?,"] # [doc = "             semi_token: input.parse()?,"] # [doc = "         })"] # [doc = "     }"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     let input = quote! {"] # [doc = " #         struct S(A, B);"] # [doc = " #     };"] # [doc = " #     syn::parse2::<TupleStruct>(input).unwrap();"] # [doc = " # }"] # [doc = " ```"] # [macro_export] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] macro_rules ! parenthesized { ($ content : ident in $ cursor : expr) => { match $ crate :: __private :: parse_parens (&$ cursor) { $ crate :: __private :: Ok (parens) => { $ content = parens . content ; parens . token } $ crate :: __private :: Err (error) => { return $ crate :: __private :: Err (error) ; } } } ; }
    };
}

parenthesized!();