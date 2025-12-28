macro_rules! Nothing {
    () => {
        # [doc = " An empty syntax tree node that consumes no tokens when parsed."] # [doc = ""] # [doc = " This is useful for attribute macros that want to ensure they are not"] # [doc = " provided any attribute args."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate proc_macro;"] # [doc = " #"] # [doc = " use proc_macro::TokenStream;"] # [doc = " use syn::parse_macro_input;"] # [doc = " use syn::parse::Nothing;"] # [doc = ""] # [doc = " # const IGNORE: &str = stringify! {"] # [doc = " #[proc_macro_attribute]"] # [doc = " # };"] # [doc = " pub fn my_attr(args: TokenStream, input: TokenStream) -> TokenStream {"] # [doc = "     parse_macro_input!(args as Nothing);"] # [doc = ""] # [doc = "     /* ... */"] # [doc = " #   TokenStream::new()"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ```text"] # [doc = " error: unexpected token"] # [doc = "  --> src/main.rs:3:19"] # [doc = "   |"] # [doc = " 3 | #[my_attr(asdf)]"] # [doc = "   |           ^^^^"] # [doc = " ```"] pub struct Nothing ;
    };
}

Nothing!()