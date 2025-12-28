macro_rules! deps {
    () => {
        PInt!();
        UTerm!();
        UInt!();
        Pow!();
        Unsigned!();
        Bit!();
        Z0!();
        NonZero!();
    };
}

macro_rules! impl_pow_i {
    () => {
        deps!();
        macro_rules ! impl_pow_i { () => () ; ($ (# [$ meta : meta]) * $ t : ty $ (, $ tail : tt) *) => ($ (# [$ meta]) * impl Pow < UTerm > for $ t { type Output = $ t ; # [inline] fn powi (self , _ : UTerm) -> Self :: Output { 1 } } $ (# [$ meta]) * impl < U : Unsigned , B : Bit > Pow < UInt < U , B >> for $ t { type Output = $ t ; # [inline] fn powi (self , _ : UInt < U , B >) -> Self :: Output { self . pow (< UInt < U , B > as Unsigned >:: to_u32 ()) } } $ (# [$ meta]) * impl Pow < Z0 > for $ t { type Output = $ t ; # [inline] fn powi (self , _ : Z0) -> Self :: Output { 1 } } $ (# [$ meta]) * impl < U : Unsigned + NonZero > Pow < PInt < U >> for $ t { type Output = $ t ; # [inline] fn powi (self , _ : PInt < U >) -> Self :: Output { self . pow (U :: to_u32 ()) } } impl_pow_i ! ($ ($ tail) ,*) ;) ; }
    };
}

impl_pow_i!();