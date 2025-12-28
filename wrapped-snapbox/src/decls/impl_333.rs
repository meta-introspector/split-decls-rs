macro_rules! deps {
    () => {
        Styled!();
        Result!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < D : std :: fmt :: Display > std :: fmt :: Display for Styled < D > { # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . style . render ()) ? ; self . display . fmt (f) ? ; write ! (f , "{}" , self . style . render_reset ()) ? ; Ok (()) } }
    };
}

impl_333!()