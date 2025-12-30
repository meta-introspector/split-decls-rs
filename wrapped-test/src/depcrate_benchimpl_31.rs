// Generated macro for impl_31 (impl)
macro_rules! Depcrate_benchimpl_31 {
() => {
// Module: crate::bench
// Provides: {"impl_31"}
// Dependencies: {}
impl Bencher { # [doc = " Callback for benchmark functions to run in their body."] pub fn iter < T , F > (& mut self , mut inner : F) where F : FnMut () -> T , { if self . mode == BenchMode :: Single { ns_iter_inner (& mut inner , 1) ; return ; } self . summary = Some (iter (& mut inner)) ; } pub fn bench < F > (& mut self , mut f : F) -> Result < Option < stats :: Summary > , String > where F : FnMut (& mut Bencher) -> Result < () , String > , { let result = f (self) ; result . map (| _ | self . summary) } }
};
}
