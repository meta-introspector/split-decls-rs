use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: mk_attr_from_item");
pub fn mk_attr_from_item (g : & AttrIdGenerator , item : AttrItem , tokens : Option < LazyAttrTokenStream > , style : AttrStyle , span : Span ,) -> Attribute { Attribute { kind : AttrKind :: Normal (Box :: new (NormalAttr { item , tokens })) , id : g . mk_attr_id () , style , span , } }
}