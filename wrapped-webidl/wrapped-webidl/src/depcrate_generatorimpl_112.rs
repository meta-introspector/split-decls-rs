// Generated macro for impl_112 (impl)
macro_rules! Depcrate_generatorimpl_112 {
() => {
// Module: crate::generator
// Provides: {"impl_112"}
// Dependencies: {}
impl ConstValue { fn generate (& self) -> TokenStream { use ConstValue :: * ; match self { Boolean (false) => quote ! (false) , Boolean (true) => quote ! (true) , Float (f) if f . is_infinite () && f . is_sign_positive () => quote ! (1.0 / 0.0) , Float (f) if f . is_infinite () && f . is_sign_negative () => quote ! (- 1.0 / 0.0) , Float (f) if f . is_nan () => quote ! (0.0 / 0.0) , Float (f) => { let f = Literal :: f64_suffixed (* f) ; quote ! (# f) } SignedInteger (i) => { let i = Literal :: i64_suffixed (* i) ; quote ! (# i) } UnsignedInteger (i) => { let i = Literal :: u64_suffixed (* i) ; quote ! (# i) } } } }
};
}
