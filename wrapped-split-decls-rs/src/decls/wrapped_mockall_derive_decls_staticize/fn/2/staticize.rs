use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn staticize (generics : & Generics) -> Generics { let mut ret = generics . clone () ; for lt in ret . lifetimes_mut () { lt . lifetime = Lifetime :: new ("'static" , Span :: call_site ()) ; } ret }
}