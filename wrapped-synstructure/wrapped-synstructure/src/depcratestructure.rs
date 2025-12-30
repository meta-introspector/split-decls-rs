// Generated macro for Structure (struct)
macro_rules! DepcrateStructure {
() => {
// Module: crate
// Provides: {"Structure"}
// Dependencies: {}
# [doc = " A wrapper around a `syn::DeriveInput` which provides utilities for creating"] # [doc = " custom derive trait implementations."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Structure < 'a > { variants : Vec < VariantInfo < 'a > > , omitted_variants : bool , ast : & 'a DeriveInput , extra_impl : Vec < GenericParam > , extra_predicates : Vec < WherePredicate > , add_bounds : AddBounds , }
};
}
