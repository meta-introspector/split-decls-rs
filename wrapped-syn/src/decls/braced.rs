macro_rules! deps {
    () => {
        ParseStream!();
        Parse!();
        Result!();
    };
}

macro_rules! braced {
    () => {
        deps!();
        # [doc = " Parse a set of curly braces and expose their content to subsequent parsers."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use quote::quote;"] # [doc = " #"] # [doc = " use syn::{braced, token, Ident, Result, Token, Type};"] # [doc = " use syn::parse::{Parse, ParseStream};"] # [doc = " use syn::punctuated::Punctuated;"] # [doc = ""] # [doc = " // Parse a simplified struct syntax like:"] # [doc = " //"] # [doc = " //     struct S {"] # [doc = " //         a: A,"] # [doc = " //         b: B,"] # [doc = " //     }"] # [doc = " struct Struct {"] # [doc = "     struct_token: Token![struct],"] # [doc = "     ident: Ident,"] # [doc = "     brace_token: token::Brace,"] # [doc = "     fields: Punctuated<Field, Token![,]>,"] # [doc = " }"] # [doc = ""] # [doc = " struct Field {"] # [doc = "     name: Ident,"] # [doc = "     colon_token: Token![:],"] # [doc = "     ty: Type,"] # [doc = " }"] # [doc = ""] # [doc = " impl Parse for Struct {"] # [doc = "     fn parse(input: ParseStream) -> Result<Self> {"] # [doc = "         let content;"] # [doc = "         Ok(Struct {"] # [doc = "             struct_token: input.parse()?,"] # [doc = "             ident: input.parse()?,"] # [doc = "             brace_token: braced!(content in input),"] # [doc = "             fields: content.parse_terminated(Field::parse, Token![,])?,"] # [doc = "         })"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " impl Parse for Field {"] # [doc = "     fn parse(input: ParseStream) -> Result<Self> {"] # [doc = "         Ok(Field {"] # [doc = "             name: input.parse()?,"] # [doc = "             colon_token: input.parse()?,"] # [doc = "             ty: input.parse()?,"] # [doc = "         })"] # [doc = "     }"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #     let input = quote! {"] # [doc = " #         struct S {"] # [doc = " #             a: A,"] # [doc = " #             b: B,"] # [doc = " #         }"] # [doc = " #     };"] # [doc = " #     syn::parse2::<Struct>(input).unwrap();"] # [doc = " # }"] # [doc = " ```"] # [macro_export] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] macro_rules ! braced { ($ content : ident in $ cursor : expr) => { match $ crate :: __private :: parse_braces (&$ cursor) { $ crate :: __private :: Ok (braces) => { $ content = braces . content ; braces . token } $ crate :: __private :: Err (error) => { return $ crate :: __private :: Err (error) ; } } } ; }
    };
}

braced!()