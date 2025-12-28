macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        impl Clone for Lifetime { fn clone (& self) -> Self { Lifetime { apostrophe : self . apostrophe , ident : self . ident . clone () , } } }
    };
}

impl_400!();