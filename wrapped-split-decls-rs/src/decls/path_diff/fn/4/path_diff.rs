use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn path_diff (from : & Path , to : & Path) -> Option < PathBuf > { let from_components : Vec < _ > = from . components () . collect () ; let to_components : Vec < _ > = to . components () . collect () ; let mut common_len = 0 ; for (a , b) in from_components . iter () . zip (to_components . iter ()) { if a == b { common_len += 1 ; } else { break ; } } let mut diff = PathBuf :: new () ; for _ in from_components . iter () . skip (common_len) { diff . push ("..") ; } for component in to_components . iter () . skip (common_len) { diff . push (component) ; } if diff . components () . next () . is_none () { Some (PathBuf :: from (".")) } else { Some (diff) } }
}