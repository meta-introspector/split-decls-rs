// Generated macro for impl_int_rem (macro)
macro_rules! Depcrate_intimpl_int_rem {
() => {
// Module: crate::int
// Provides: {"impl_int_rem"}
// Dependencies: {}
macro_rules ! impl_int_rem { ($ A : ident , $ B : ident , $ R : ident) => { # [doc = " `$A<Ul> % $B<Ur> = $R<Ul % Ur>`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Rem <$ B < Ur >> for $ A < Ul > where Ul : Rem < Ur >, $ A < Ul >: PrivateRem << Ul as Rem < Ur >>:: Output , $ B < Ur >>, { type Output = <$ A < Ul > as PrivateRem << Ul as Rem < Ur >>:: Output , $ B < Ur >>>:: Output ; # [inline] fn rem (self , rhs : $ B < Ur >) -> Self :: Output { self . private_rem (self . n % rhs . n , rhs) } } impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > PrivateRem < U0 , $ B < Ur >> for $ A < Ul > { type Output = Z0 ; # [inline] fn private_rem (self , _ : U0 , _ : $ B < Ur >) -> Self :: Output { Z0 } } impl < Ul , Ur , U , B > PrivateRem < UInt < U , B >, $ B < Ur >> for $ A < Ul > where Ul : Unsigned + NonZero , Ur : Unsigned + NonZero , U : Unsigned , B : Bit , { type Output = $ R < UInt < U , B >>; # [inline] fn private_rem (self , urem : UInt < U , B >, _ : $ B < Ur >) -> Self :: Output { $ R { n : urem } } } } ; }
};
}
