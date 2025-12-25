use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn derive_try_from_bytes_enum(
    ast: &DeriveInput,
    enm: &DataEnum,
    top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = EnumRepr::from_attrs(&ast.attrs)?;
    let could_be_from_bytes = enum_size_from_repr(&repr)
        .map(|size| enm.fields().is_empty() && enm.variants.len() == 1usize << size)
        .unwrap_or(false);
    let trivial_is_bit_valid = try_gen_trivial_is_bit_valid(
        ast,
        top_level,
        zerocopy_crate,
    );
    let extra = match (trivial_is_bit_valid, could_be_from_bytes) {
        (Some(is_bit_valid), _) => is_bit_valid,
        (None, true) => unsafe { gen_trivial_is_bit_valid_unchecked(zerocopy_crate) }
        (None, false) => {
            r#enum::derive_is_bit_valid(
                &ast.ident,
                &repr,
                &ast.generics,
                enm,
                zerocopy_crate,
            )?
        }
    };
    Ok(
        ImplBlockBuilder::new(
                ast,
                enm,
                Trait::TryFromBytes,
                FieldBounds::ALL_SELF,
                zerocopy_crate,
            )
            .inner_extras(extra)
            .build(),
    )
}
