macro_rules! parse_in {
    () => {
        # [doc = " Runs the given subparser `f` on the tokens of the given `attr`'s item."] pub fn parse_in < 'a , T > (psess : & 'a ParseSess , tts : TokenStream , name : & 'static str , mut f : impl FnMut (& mut Parser < 'a >) -> PResult < 'a , T > ,) -> PResult < 'a , T > { let mut parser = Parser :: new (psess , tts , Some (name)) ; let result = f (& mut parser) ? ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok (result) }
    };
}

parse_in!()