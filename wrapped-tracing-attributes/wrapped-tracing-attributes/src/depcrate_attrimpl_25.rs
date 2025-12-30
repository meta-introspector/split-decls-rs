// Generated macro for impl_25 (impl)
macro_rules! Depcrate_attrimpl_25 {
() => {
// Module: crate::attr
// Provides: {"impl_25"}
// Dependencies: {}
impl < T : Parse > Parse for StrArg < T > { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let _ = input . parse :: < T > () ? ; let _ = input . parse :: < Token ! [=] > () ? ; let value = input . parse () ? ; Ok (Self { value , _p : std :: marker :: PhantomData , }) } }
};
}
