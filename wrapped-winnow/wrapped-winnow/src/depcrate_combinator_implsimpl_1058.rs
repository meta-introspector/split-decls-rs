// Generated macro for impl_1058 (impl)
macro_rules! Depcrate_combinator_implsimpl_1058 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1058"}
// Dependencies: {}
impl < F , G , I , O , O2 , E > Parser < I , O2 , E > for Map < F , G , I , O , O2 , E > where F : Parser < I , O , E > , G : FnMut (O) -> O2 , { # [inline] fn parse_next (& mut self , i : & mut I) -> Result < O2 , E > { match self . parser . parse_next (i) { Err (e) => Err (e) , Ok (o) => Ok ((self . map) (o)) , } } }
};
}
