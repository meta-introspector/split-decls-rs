macro_rules! deps {
    () => {
        Lifetime!();
        Punctuated!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl LifetimeParam { pub fn new (lifetime : Lifetime) -> Self { LifetimeParam { attrs : Vec :: new () , lifetime , colon_token : None , bounds : Punctuated :: new () , } } }
    };
}

impl_319!()