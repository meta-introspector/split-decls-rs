use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Output array of [`OutputSizeUser`] implementors."] pub type Output < T > = Array < u8 , OutputSize < T > > ;
}