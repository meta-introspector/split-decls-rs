use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn mk_attr (g : & AttrIdGenerator , style : AttrStyle , unsafety : Safety , path : Path , args : AttrArgs , span : Span ,) -> Attribute { mk_attr_from_item (g , AttrItem { unsafety , path , args , tokens : None } , None , style , span) }