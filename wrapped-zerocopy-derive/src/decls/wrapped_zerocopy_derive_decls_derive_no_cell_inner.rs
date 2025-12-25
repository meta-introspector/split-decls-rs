use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn derive_no_cell_inner(
    ast: &DeriveInput,
    _top_level: Trait,
    zerocopy_crate: &Path,
) -> TokenStream {
    match &ast.data {
        Data::Struct(strct) => {
            ImplBlockBuilder::new(
                    ast,
                    strct,
                    Trait::Immutable,
                    FieldBounds::ALL_SELF,
                    zerocopy_crate,
                )
                .build()
        }
        Data::Enum(enm) => {
            ImplBlockBuilder::new(
                    ast,
                    enm,
                    Trait::Immutable,
                    FieldBounds::ALL_SELF,
                    zerocopy_crate,
                )
                .build()
        }
        Data::Union(unn) => {
            ImplBlockBuilder::new(
                    ast,
                    unn,
                    Trait::Immutable,
                    FieldBounds::ALL_SELF,
                    zerocopy_crate,
                )
                .build()
        }
    }
}
