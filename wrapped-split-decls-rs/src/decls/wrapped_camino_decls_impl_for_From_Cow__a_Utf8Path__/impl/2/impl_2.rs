use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > From < Cow < 'a , Utf8Path > > for Utf8PathBuf { fn from (path : Cow < 'a , Utf8Path >) -> Utf8PathBuf { path . into_owned () } }