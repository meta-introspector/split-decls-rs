use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This is a more flexible/imprecise `ItemFn` type,
/// which's block is just a `TokenStream` (it may contain invalid code).
#[derive(Debug, Clone)]
struct MaybeItemFn {
    outer_attrs: Vec<Attribute>,
    inner_attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    brace_token: Brace,
    block: TokenStream,
}
