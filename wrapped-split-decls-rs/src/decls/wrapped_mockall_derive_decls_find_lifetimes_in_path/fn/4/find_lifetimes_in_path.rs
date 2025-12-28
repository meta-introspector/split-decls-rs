use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn find_lifetimes_in_path (path : & Path) -> HashSet < Lifetime > { let mut ret = HashSet :: default () ; for seg in path . segments . iter () { if let PathArguments :: AngleBracketed (abga) = & seg . arguments { for arg in abga . args . iter () { match arg { GenericArgument :: Lifetime (lt) => { ret . insert (lt . clone ()) ; } GenericArgument :: Type (ty) => { ret . extend (find_lifetimes (ty)) ; } GenericArgument :: AssocType (at) => { ret . extend (find_lifetimes (& at . ty)) ; } GenericArgument :: Constraint (c) => { for bound in c . bounds . iter () { ret . extend (find_lifetimes_in_tpb (bound)) ; } } GenericArgument :: Const (_) => { } _ => { } } } } } ret }
}