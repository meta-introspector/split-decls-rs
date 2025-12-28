macro_rules! ident_from_token {
    () => {
        macro_rules ! ident_from_token { ($ token : ident) => { impl From < Token ! [$ token] > for Ident { fn from (token : Token ! [$ token]) -> Ident { Ident :: new (stringify ! ($ token) , token . span) } } } ; }
    };
}

ident_from_token!();