use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Extend < CfgAtom > for CfgOptions { fn extend < T : IntoIterator < Item = CfgAtom > > (& mut self , iter : T) { iter . into_iter () . for_each (| cfg_flag | self . insert_any_atom (cfg_flag)) ; } }
}