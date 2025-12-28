use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [doc = " *Requires Rust 1.68 or newer.*"] # [cfg (path_buf_deref_mut)] # [allow (clippy :: incompatible_msrv)] impl std :: ops :: DerefMut for Utf8PathBuf { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { Utf8Path :: assume_utf8_mut (& mut self . 0) } } }
}