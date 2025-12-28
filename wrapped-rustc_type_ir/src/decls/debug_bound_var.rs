macro_rules! debug_bound_var {
    () => {
        pub fn debug_bound_var < T : std :: fmt :: Write > (fmt : & mut T , debruijn : DebruijnIndex , var : impl std :: fmt :: Debug ,) -> Result < () , std :: fmt :: Error > { if debruijn == INNERMOST { write ! (fmt , "^{var:?}") } else { write ! (fmt , "^{}_{:?}" , debruijn . index () , var) } }
    };
}

debug_bound_var!();