use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn scope_path < 'a , R : LookupSpan < 'a > > (span : & SpanRef < 'a , R >) -> ScopeFromRoot < 'a , R > { span . scope () . from_root () }