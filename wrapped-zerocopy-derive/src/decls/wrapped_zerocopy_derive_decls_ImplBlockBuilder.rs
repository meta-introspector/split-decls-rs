use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct ImplBlockBuilder<'a, D: DataExt> {
    input: &'a DeriveInput,
    data: &'a D,
    trt: Trait,
    field_type_trait_bounds: FieldBounds<'a>,
    zerocopy_crate: &'a Path,
    self_type_trait_bounds: SelfBounds<'a>,
    padding_check: Option<PaddingCheck>,
    inner_extras: Option<TokenStream>,
    outer_extras: Option<TokenStream>,
}
