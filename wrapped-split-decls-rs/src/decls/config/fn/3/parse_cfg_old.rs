use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn parse_cfg_old < 'a > (meta_item : & 'a ast :: MetaItem , sess : & Session) -> Option < & 'a ast :: MetaItemInner > { let span = meta_item . span ; match meta_item . meta_item_list () { None => { None } Some ([]) => { None } Some ([_ , .. , l]) => { None } Some ([single]) => match single . meta_item_or_bool () { Some (meta_item) => Some (meta_item) , None => { None } } } }