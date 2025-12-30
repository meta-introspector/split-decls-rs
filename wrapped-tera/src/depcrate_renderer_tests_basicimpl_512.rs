// Generated macro for impl_512 (impl)
macro_rules! Depcrate_renderer_tests_basicimpl_512 {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"impl_512"}
// Dependencies: {}
impl Function for Next { fn call (& self , _args : & HashMap < String , Value >) -> Result < Value > { Ok (Value :: Number (self . 0 . fetch_add (1 , Ordering :: Relaxed) . into ())) } }
};
}
