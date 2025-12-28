use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Defines system native-endian serialization."] # [doc = ""] # [doc = " Note that this type has no value constructor. It is used purely at the"] # [doc = " type level."] # [doc = ""] # [doc = " On this platform, this is an alias for [`BigEndian`]."] # [doc = ""] # [doc = " [`BigEndian`]: enum.BigEndian.html"] # [cfg (target_endian = "big")] pub type NativeEndian = BigEndian ;