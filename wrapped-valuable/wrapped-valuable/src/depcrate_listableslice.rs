// Generated macro for slice (macro)
macro_rules! Depcrate_listableslice {
() => {
// Module: crate::listable
// Provides: {"slice"}
// Dependencies: {}
macro_rules ! slice { ($ ($ (# [$ attrs : meta]) * ($ ($ generics : tt) *) $ ty : ty ,) *) => { $ ($ (# [$ attrs]) * impl <$ ($ generics) *> Valuable for $ ty { fn as_value (& self) -> Value <'_ > { Value :: Listable (self as & dyn Listable) } fn visit (& self , visit : & mut dyn Visit) { T :: visit_slice (self , visit) ; } } $ (# [$ attrs]) * impl <$ ($ generics) *> Listable for $ ty { fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }) * } ; }
};
}
