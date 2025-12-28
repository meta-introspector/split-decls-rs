use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FromIterator < CfgAtom > for CfgOptions { fn from_iter < T : IntoIterator < Item = CfgAtom > > (iter : T) -> Self { let mut options = CfgOptions :: default () ; options . extend (iter) ; options } }
}