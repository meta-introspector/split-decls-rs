// Generated macro for supertraits (function)
macro_rules! Depcrate_elaboratesupertraits {
() => {
// Module: crate::elaborate
// Provides: {"supertraits"}
// Dependencies: {}
pub fn supertraits < I : Interner > (cx : I , trait_ref : ty :: Binder < I , ty :: TraitRef < I > > ,) -> FilterToTraits < I , Elaborator < I , I :: Clause > > { elaborate (cx , [trait_ref . upcast (cx)]) . filter_only_self () . filter_to_traits () }
};
}
