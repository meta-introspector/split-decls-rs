// Generated macro for collection (macro)
macro_rules! Depcrate_listablecollection {
() => {
// Module: crate::listable
// Provides: {"collection"}
// Dependencies: {}
macro_rules ! collection { ($ ($ (# [$ attrs : meta]) * ($ ($ generics : tt) *) $ ty : ty ,) *) => { $ ($ (# [$ attrs]) * impl <$ ($ generics) *> Valuable for $ ty { fn as_value (& self) -> Value <'_ > { Value :: Listable (self as & dyn Listable) } fn visit (& self , visit : & mut dyn Visit) { for value in self . iter () { visit . visit_value (value . as_value ()) ; } } } $ (# [$ attrs]) * impl <$ ($ generics) *> Listable for $ ty { fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }) * } ; }
};
}
