// Generated macro for phantom_type (macro)
macro_rules! Depcrate_variancephantom_type {
() => {
// Module: crate::variance
// Provides: {"phantom_type"}
// Dependencies: {}
macro_rules ! phantom_type { ($ ($ (# [$ attr : meta]) * pub struct $ name : ident <$ t : ident > ($ ($ inner : tt) *) ;) *) => { $ ($ (# [$ attr]) * pub struct $ name <$ t > ($ ($ inner) *) where T : ? Sized ; impl < T > $ name < T > where T : ? Sized { # [doc = " Constructs a new instance of the variance marker."] pub const fn new () -> Self { Self (PhantomData) } } impl < T > self :: sealed :: Sealed for $ name < T > where T : ? Sized { const VALUE : Self = Self :: new () ; } impl < T > Variance for $ name < T > where T : ? Sized { } impl < T > Default for $ name < T > where T : ? Sized { fn default () -> Self { Self (PhantomData) } } impl < T > fmt :: Debug for $ name < T > where T : ? Sized { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { write ! (f , "{}<{}>" , stringify ! ($ name) , type_name ::< T > ()) } } impl < T > Clone for $ name < T > where T : ? Sized { fn clone (& self) -> Self { * self } } impl < T > Copy for $ name < T > where T : ? Sized { } impl < T > PartialEq for $ name < T > where T : ? Sized { fn eq (& self , _ : & Self) -> bool { true } } impl < T > Eq for $ name < T > where T : ? Sized { } # [allow (clippy :: non_canonical_partial_ord_impl)] impl < T > PartialOrd for $ name < T > where T : ? Sized { fn partial_cmp (& self , _ : & Self) -> Option < Ordering > { Some (Ordering :: Equal) } } impl < T > Ord for $ name < T > where T : ? Sized { fn cmp (& self , _ : & Self) -> Ordering { Ordering :: Equal } } impl < T > Hash for $ name < T > where T : ? Sized { fn hash < H : Hasher > (& self , _ : & mut H) { } }) * } ; }
};
}
