macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_low_level_token {
    () => {
        deps!();
        macro_rules ! impl_low_level_token { ($ display : literal $ ($ path : ident) ::+ $ get : ident) => { # [cfg (feature = "parsing")] impl Token for $ ($ path) ::+ { fn peek (cursor : Cursor) -> bool { cursor .$ get () . is_some () } fn display () -> &'static str { $ display } } # [cfg (feature = "parsing")] impl private :: Sealed for $ ($ path) ::+ { } } ; }
    };
}

impl_low_level_token!()