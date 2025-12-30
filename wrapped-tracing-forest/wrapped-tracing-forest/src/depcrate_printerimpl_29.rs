// Generated macro for impl_29 (impl)
macro_rules! Depcrate_printerimpl_29 {
() => {
// Module: crate::printer
// Provides: {"impl_29"}
// Dependencies: {}
impl < F , E > Formatter for F where F : Fn (& Tree) -> Result < String , E > , E : Error + Send + Sync , { type Error = E ; # [inline] fn fmt (& self , tree : & Tree) -> Result < String , E > { self (tree) } }
};
}
