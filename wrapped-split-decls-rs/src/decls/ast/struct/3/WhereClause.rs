use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A where-clause in a definition."] # [derive (Clone , Encodable , Decodable , Debug , Default , Walkable)] pub struct WhereClause { # [doc = " `true` if we ate a `where` token."] # [doc = ""] # [doc = " This can happen if we parsed no predicates, e.g., `struct Foo where {}`."] # [doc = " This allows us to pretty-print accurately and provide correct suggestion diagnostics."] pub has_where_token : bool , pub predicates : ThinVec < WherePredicate > , pub span : Span , }