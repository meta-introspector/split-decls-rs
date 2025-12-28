macro_rules! deps {
    () => {
        Release!();
        Bound!();
        Date!();
    };
}

macro_rules! Expr {
    () => {
        deps!();
        pub enum Expr { Stable , Beta , Nightly , Date (Date) , Since (Bound) , Before (Bound) , Release (Release) , Not (Box < Expr >) , Any (Vec < Expr >) , All (Vec < Expr >) , }
    };
}

Expr!();