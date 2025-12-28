use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct PolyTraitRef { # [doc = " The `'a` in `for<'a> Foo<&'a T>`."] pub bound_generic_params : ThinVec < GenericParam > , pub modifiers : TraitBoundModifiers , # [doc = " The `Foo<&'a T>` in `<'a> Foo<&'a T>`."] pub trait_ref : TraitRef , pub span : Span , # [doc = " When `Yes`, the first and last character of `span` are an opening"] # [doc = " and a closing paren respectively."] pub parens : Parens , }
}