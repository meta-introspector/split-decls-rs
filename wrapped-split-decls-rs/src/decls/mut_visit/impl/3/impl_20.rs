use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < V : MutVisitor , T1 , T2 , T3 > MutVisitable < V > for (T1 , T2 , T3) where T1 : MutVisitable < V , Extra = () > , T2 : MutVisitable < V , Extra = () > , T3 : MutVisitable < V , Extra = () > , { type Extra = () ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { self . 0 . visit_mut (visitor , extra) ; self . 1 . visit_mut (visitor , extra) ; self . 2 . visit_mut (visitor , extra) ; } }
}