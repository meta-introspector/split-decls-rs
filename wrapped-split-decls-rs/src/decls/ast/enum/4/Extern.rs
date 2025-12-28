use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " `extern` qualifier on a function item or function type."] # [derive (Clone , Copy , Encodable , Decodable , Debug , Walkable)] pub enum Extern { # [doc = " No explicit extern keyword was used."] # [doc = ""] # [doc = " E.g. `fn foo() {}`."] None , # [doc = " An explicit extern keyword was used, but with implicit ABI."] # [doc = ""] # [doc = " E.g. `extern fn foo() {}`."] # [doc = ""] # [doc = " This is just `extern \"C\"` (see `rustc_abi::ExternAbi::FALLBACK`)."] Implicit (Span) , # [doc = " An explicit extern keyword was used with an explicit ABI."] # [doc = ""] # [doc = " E.g. `extern \"C\" fn foo() {}`."] Explicit (StrLit , Span) , }
}