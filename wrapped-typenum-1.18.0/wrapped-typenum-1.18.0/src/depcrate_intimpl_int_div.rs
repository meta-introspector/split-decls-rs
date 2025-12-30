// Generated macro for impl_int_div (macro)
macro_rules! Depcrate_intimpl_int_div {
() => {
// Module: crate::int
// Provides: {"impl_int_div"}
// Dependencies: {}
macro_rules ! impl_int_div { ($ A : ident , $ B : ident , $ R : ident) => { # [doc = " `$A<Ul> / $B<Ur> = $R<Ul / Ur>`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Div <$ B < Ur >> for $ A < Ul > where Ul : Cmp < Ur >, $ A < Ul >: PrivateDivInt << Ul as Cmp < Ur >>:: Output , $ B < Ur >>, { type Output = <$ A < Ul > as PrivateDivInt << Ul as Cmp < Ur >>:: Output , $ B < Ur >>>:: Output ; # [inline] fn div (self , rhs : $ B < Ur >) -> Self :: Output { let lhs_cmp_rhs = self . n . compare ::< Internal > (& rhs . n) ; self . private_div_int (lhs_cmp_rhs , rhs) } } impl < Ul , Ur > PrivateDivInt < Less , $ B < Ur >> for $ A < Ul > where Ul : Unsigned + NonZero , Ur : Unsigned + NonZero , { type Output = Z0 ; # [inline] fn private_div_int (self , _ : Less , _ : $ B < Ur >) -> Self :: Output { Z0 } } impl < Ul , Ur > PrivateDivInt < Equal , $ B < Ur >> for $ A < Ul > where Ul : Unsigned + NonZero , Ur : Unsigned + NonZero , { type Output = $ R < U1 >; # [inline] fn private_div_int (self , _ : Equal , _ : $ B < Ur >) -> Self :: Output { $ R { n : U1 :: new () } } } impl < Ul , Ur > PrivateDivInt < Greater , $ B < Ur >> for $ A < Ul > where Ul : Unsigned + NonZero + Div < Ur >, Ur : Unsigned + NonZero , < Ul as Div < Ur >>:: Output : Unsigned + NonZero , { type Output = $ R << Ul as Div < Ur >>:: Output >; # [inline] fn private_div_int (self , _ : Greater , d : $ B < Ur >) -> Self :: Output { $ R { n : self . n / d . n } } } } ; }
};
}
