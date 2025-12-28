macro_rules! deps {
    () => {
        Cursor!();
        ParseStream!();
        Result!();
        Parse!();
    };
}

macro_rules! define_punctuation {
    () => {
        deps!();
        macro_rules ! define_punctuation { ($ ($ token : literal pub struct $ name : ident /$ len : tt # [doc = $ usage : literal]) *) => { $ (define_punctuation_structs ! { $ token pub struct $ name /$ len # [doc = $ usage] } # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for $ name { fn to_tokens (& self , tokens : & mut TokenStream) { printing :: punct ($ token , & self . spans , tokens) ; } } # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for $ name { fn parse (input : ParseStream) -> Result < Self > { Ok ($ name { spans : parsing :: punct (input , $ token) ?, }) } } # [cfg (feature = "parsing")] impl Token for $ name { fn peek (cursor : Cursor) -> bool { parsing :: peek_punct (cursor , $ token) } fn display () -> &'static str { concat ! ("`" , $ token , "`") } } # [cfg (feature = "parsing")] impl private :: Sealed for $ name { }) * } ; }
    };
}

define_punctuation!()