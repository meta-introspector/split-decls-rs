use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < V : MutVisitor , T > MutVisitable < V > for Spanned < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { let Spanned { span , node } = self ; span . visit_mut (visitor , ()) ; node . visit_mut (visitor , extra) ; } }
}