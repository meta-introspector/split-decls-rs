// Generated macro for VariantInfo (struct)
macro_rules! DepcrateVariantInfo {
() => {
// Module: crate
// Provides: {"VariantInfo"}
// Dependencies: {}
# [doc = " A wrapper around a `syn::DeriveInput`'s variant which provides utilities"] # [doc = " for destructuring `Variant`s with `match` expressions."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct VariantInfo < 'a > { pub prefix : Option < & 'a Ident > , bindings : Vec < BindingInfo < 'a > > , ast : VariantAst < 'a > , generics : & 'a Generics , original_length : usize , }
};
}
