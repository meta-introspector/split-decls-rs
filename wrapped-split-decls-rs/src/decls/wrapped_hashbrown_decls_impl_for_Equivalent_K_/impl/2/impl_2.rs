use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (not (feature = "equivalent"))] impl < Q : ? Sized , K : ? Sized > Equivalent < K > for Q where Q : Eq , K : core :: borrow :: Borrow < Q > , { fn equivalent (& self , key : & K) -> bool { self == key . borrow () } }
}