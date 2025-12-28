use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < V : MutVisitor , T > MutVisitable < V > for Vec < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { for item in self { item . visit_mut (visitor , extra) ; } } }
}