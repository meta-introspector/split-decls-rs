use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I , T > From < I > for RawArgs where I : Iterator < Item = T > , T : Into < OsString > , { fn from (val : I) -> Self { Self { items : val . map (| x | x . into ()) . collect () , } } }
}