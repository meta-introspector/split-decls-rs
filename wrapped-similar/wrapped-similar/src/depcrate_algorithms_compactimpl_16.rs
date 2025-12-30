// Generated macro for impl_16 (impl)
macro_rules! Depcrate_algorithms_compactimpl_16 {
() => {
// Module: crate::algorithms::compact
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'old , 'new , Old , New , D > Compact < 'old , 'new , Old , New , D > where D : DiffHook , Old : Index < usize > + ? Sized + 'old , New : Index < usize > + ? Sized + 'new , New :: Output : PartialEq < Old :: Output > , { # [doc = " Creates a new compact hook wrapping another hook."] pub fn new (d : D , old : & 'old Old , new : & 'new New) -> Self { Compact { d , ops : Vec :: new () , old , new , } } # [doc = " Extracts the inner hook."] pub fn into_inner (self) -> D { self . d } }
};
}
