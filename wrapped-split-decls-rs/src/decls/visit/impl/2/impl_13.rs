use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , V : Visitor < 'a > , T : ? Sized > Visitable < 'a , V > for Box < T > where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { (* * self) . visit (visitor , extra) } }
}