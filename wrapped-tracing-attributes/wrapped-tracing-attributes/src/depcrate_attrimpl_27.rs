// Generated macro for impl_27 (impl)
macro_rules! Depcrate_attrimpl_27 {
() => {
// Module: crate::attr
// Provides: {"impl_27"}
// Dependencies: {}
impl < T : Parse > Parse for ExprArg < T > { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let _ = input . parse :: < T > () ? ; let _ = input . parse :: < Token ! [=] > () ? ; let value = input . parse () ? ; Ok (Self { value , _p : std :: marker :: PhantomData , }) } }
};
}
