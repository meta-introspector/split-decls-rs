use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: fetch_generics");
fn fetch_generics < 'a > (set : & [bool] , generics : & 'a Generics) -> Vec < & 'a Ident > { let mut tys = vec ! [] ; for (& seen , param) in set . iter () . zip (generics . params . iter ()) { if seen { if let GenericParam :: Type (tparam) = param { tys . push (& tparam . ident) ; } } } tys }
}