use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > From < Utf8PathBuf > for Cow < 'a , Path > { fn from (path : Utf8PathBuf) -> Cow < 'a , Path > { PathBuf :: from (path) . into () } }
}