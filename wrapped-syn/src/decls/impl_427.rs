macro_rules! deps {
    () => {
        ParseStream!();
        Error!();
        Result!();
        Parse!();
        LitIntRepr!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl LitInt { # [track_caller] pub fn new (repr : & str , span : Span) -> Self { let (digits , suffix) = match value :: parse_lit_int (repr) { Some (parse) => parse , None => panic ! ("not an integer literal: `{}`" , repr) , } ; let mut token : Literal = repr . parse () . unwrap () ; token . set_span (span) ; LitInt { repr : Box :: new (LitIntRepr { token , digits , suffix , }) , } } pub fn base10_digits (& self) -> & str { & self . repr . digits } # [doc = " Parses the literal into a selected number type."] # [doc = ""] # [doc = " This is equivalent to `lit.base10_digits().parse()` except that the"] # [doc = " resulting errors will be correctly spanned to point to the literal token"] # [doc = " in the macro input."] # [doc = ""] # [doc = " ```"] # [doc = " use syn::LitInt;"] # [doc = " use syn::parse::{Parse, ParseStream, Result};"] # [doc = ""] # [doc = " struct Port {"] # [doc = "     value: u16,"] # [doc = " }"] # [doc = ""] # [doc = " impl Parse for Port {"] # [doc = "     fn parse(input: ParseStream) -> Result<Self> {"] # [doc = "         let lit: LitInt = input.parse()?;"] # [doc = "         let value = lit.base10_parse::<u16>()?;"] # [doc = "         Ok(Port { value })"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub fn base10_parse < N > (& self) -> Result < N > where N : FromStr , N :: Err : Display , { self . base10_digits () . parse () . map_err (| err | Error :: new (self . span () , err)) } pub fn suffix (& self) -> & str { & self . repr . suffix } pub fn span (& self) -> Span { self . repr . token . span () } pub fn set_span (& mut self , span : Span) { self . repr . token . set_span (span) ; } pub fn token (& self) -> Literal { self . repr . token . clone () } }
    };
}

impl_427!()