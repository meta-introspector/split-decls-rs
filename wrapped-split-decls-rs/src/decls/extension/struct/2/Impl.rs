use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Impl { attrs : Vec < Attribute > , generics : Generics , self_ty : Type , items : Vec < ImplItem > , wc : Option < WhereClause > , }
}