macro_rules! deps {
    () => {
        UInt!();
        NInt!();
        Pow!();
        Z0!();
        UTerm!();
        Unsigned!();
        PInt!();
        Bit!();
        NonZero!();
    };
}

macro_rules! impl_pow_f {
    () => {
        deps!();
        macro_rules ! impl_pow_f { ($ t : ty) => { impl Pow < UTerm > for $ t { type Output = $ t ; # [inline] fn powi (self , _ : UTerm) -> Self :: Output { 1.0 } } impl < U : Unsigned , B : Bit > Pow < UInt < U , B >> for $ t { type Output = $ t ; # [inline] fn powi (self , _ : UInt < U , B >) -> Self :: Output { let mut exp = < UInt < U , B > as Unsigned >:: to_u32 () ; let mut base = self ; if exp == 0 { return 1.0 ; } while exp & 1 == 0 { base *= base ; exp >>= 1 ; } if exp == 1 { return base ; } let mut acc = base . clone () ; while exp > 1 { exp >>= 1 ; base *= base ; if exp & 1 == 1 { acc *= base . clone () ; } } acc } } impl Pow < Z0 > for $ t { type Output = $ t ; # [inline] fn powi (self , _ : Z0) -> Self :: Output { 1.0 } } impl < U : Unsigned + NonZero > Pow < PInt < U >> for $ t { type Output = $ t ; # [inline] fn powi (self , _ : PInt < U >) -> Self :: Output { let mut exp = U :: to_u32 () ; let mut base = self ; if exp == 0 { return 1.0 ; } while exp & 1 == 0 { base *= base ; exp >>= 1 ; } if exp == 1 { return base ; } let mut acc = base . clone () ; while exp > 1 { exp >>= 1 ; base *= base ; if exp & 1 == 1 { acc *= base . clone () ; } } acc } } impl < U : Unsigned + NonZero > Pow < NInt < U >> for $ t { type Output = $ t ; # [inline] fn powi (self , _ : NInt < U >) -> Self :: Output { <$ t as Pow < PInt < U >>>:: powi (self , PInt :: new ()) . recip () } } } ; }
    };
}

impl_pow_f!();