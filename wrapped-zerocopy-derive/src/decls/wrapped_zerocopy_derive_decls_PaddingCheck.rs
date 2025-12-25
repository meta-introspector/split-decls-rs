use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This enum describes what kind of padding check needs to be generated for the
/// associated impl.
enum PaddingCheck {
    /// Check that the sum of the fields' sizes exactly equals the struct's
    /// size.
    Struct,
    /// Check that a `repr(C)` struct has no padding.
    ReprCStruct,
    /// Check that the size of each field exactly equals the union's size.
    Union,
    /// Check that every variant of the enum contains no padding.
    ///
    /// Because doing so requires a tag enum, this padding check requires an
    /// additional `TokenStream` which defines the tag enum as `___ZerocopyTag`.
    Enum { tag_type_definition: TokenStream },
}
