use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for ByteSize { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let display = self . display () ; if f . width () . is_none () { fmt :: Display :: fmt (& display , f) } else { f . pad (& display . to_string ()) } } }
}