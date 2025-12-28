use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A constraint on an associated item."] # [doc = ""] # [doc = " ### Examples"] # [doc = ""] # [doc = " * the `A = Ty` and `B = Ty` in `Trait<A = Ty, B = Ty>`"] # [doc = " * the `G<Ty> = Ty` in `Trait<G<Ty> = Ty>`"] # [doc = " * the `A: Bound` in `Trait<A: Bound>`"] # [doc = " * the `RetTy` in `Trait(ArgTy, ArgTy) -> RetTy`"] # [doc = " * the `C = { Ct }` in `Trait<C = { Ct }>` (feature `associated_const_equality`)"] # [doc = " * the `f(..): Bound` in `Trait<f(..): Bound>` (feature `return_type_notation`)"] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct AssocItemConstraint { pub id : NodeId , pub ident : Ident , pub gen_args : Option < GenericArgs > , pub kind : AssocItemConstraintKind , pub span : Span , }
}