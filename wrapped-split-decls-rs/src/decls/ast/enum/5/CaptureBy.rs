use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " A capture clause used in closures and `async` blocks."] # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic , Walkable)] pub enum CaptureBy { # [doc = " `move |x| y + x`."] Value { # [doc = " The span of the `move` keyword."] move_kw : Span , } , # [doc = " `move` or `use` keywords were not specified."] Ref , # [doc = " `use |x| y + x`."] # [doc = ""] # [doc = " Note that if you have a regular closure like `|| x.use`, this will *not* result"] # [doc = " in a `Use` capture. Instead, the `ExprUseVisitor` will look at the type"] # [doc = " of `x` and treat `x.use` as either a copy/clone/move as appropriate."] Use { # [doc = " The span of the `use` keyword."] use_kw : Span , } , }
}