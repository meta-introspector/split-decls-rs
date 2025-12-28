use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , V : Visitor < 'a > , T > Visitable < 'a , V > for Spanned < T > where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { let Spanned { span : _ , node } = self ; node . visit (visitor , extra) } }
}