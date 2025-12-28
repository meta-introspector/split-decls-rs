macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Span { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let state = if self . is_insert () { "inserted" } else { "replaced" } ; let committed = if self . committed { "committed" } else { "uncommitted" } ; write ! (f , "({}, {}: {state}, {committed})" , self . range . start , self . range . end) } }
    };
}

impl_10!()