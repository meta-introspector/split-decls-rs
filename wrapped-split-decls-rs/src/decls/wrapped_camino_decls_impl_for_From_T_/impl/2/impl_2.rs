use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : ? Sized + AsRef < str > > From < & T > for Box < Utf8Path > { fn from (s : & T) -> Box < Utf8Path > { Utf8PathBuf :: from (s) . into_boxed_path () } }