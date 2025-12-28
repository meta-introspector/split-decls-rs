macro_rules! deps {
    () => {
        Error!();
        Decor!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Decor { # [inline] fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut d = formatter . debug_struct ("Decor") ; match & self . prefix { Some (r) => d . field ("prefix" , r) , None => d . field ("prefix" , & "default") , } ; match & self . suffix { Some (r) => d . field ("suffix" , r) , None => d . field ("suffix" , & "default") , } ; d . finish () } }
    };
}

impl_223!();