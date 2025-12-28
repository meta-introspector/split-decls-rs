use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (all (feature = "futures-01" , feature = "std"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "futures-01" , feature = "std"))))] impl < T : futures_01 :: Future > futures_01 :: Future for WithDispatch < T > { type Item = T :: Item ; type Error = T :: Error ; fn poll (& mut self) -> futures_01 :: Poll < Self :: Item , Self :: Error > { let inner = & mut self . inner ; dispatcher :: with_default (& self . dispatch , | | inner . poll ()) } }
}