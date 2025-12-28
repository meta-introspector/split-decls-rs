macro_rules! deps {
    () => {
        Gcd!();
    };
}

macro_rules! Gcf {
    () => {
        deps!();
        # [doc = " Alias for the associated type of `Gcd`: `Gcf<A, B> = <A as Gcd<B>>::Output>`"] pub type Gcf < A , B > = < A as Gcd < B > > :: Output ;
    };
}

Gcf!();