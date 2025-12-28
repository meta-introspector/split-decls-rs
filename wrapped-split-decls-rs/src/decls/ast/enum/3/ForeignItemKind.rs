use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An item in `extern` block."] # [derive (Clone , Encodable , Decodable , Debug)] pub enum ForeignItemKind { # [doc = " A foreign static item (`static FOO: u8`)."] Static (Box < StaticItem >) , # [doc = " A foreign function."] Fn (Box < Fn >) , # [doc = " A foreign type."] TyAlias (Box < TyAlias >) , # [doc = " A macro expanding to foreign items."] MacCall (Box < MacCall >) , }