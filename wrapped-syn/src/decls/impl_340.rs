macro_rules! impl_340 {
    () => {
        impl From < Token ! [_] > for Ident { fn from (token : Token ! [_]) -> Ident { Ident :: new ("_" , token . span) } }
    };
}

impl_340!()