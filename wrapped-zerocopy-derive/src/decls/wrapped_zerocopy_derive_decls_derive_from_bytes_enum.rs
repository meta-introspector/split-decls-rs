use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enum is `FromBytes` if:
/// - Every possible bit pattern must be valid, which means that every bit
///   pattern must correspond to a different enum variant. Thus, for an enum
///   whose layout takes up N bytes, there must be 2^N variants.
/// - Since we must know N, only representations which guarantee the layout's
///   size are allowed. These are `repr(uN)` and `repr(iN)` (`repr(C)` implies an
///   implementation-defined size). `usize` and `isize` technically guarantee the
///   layout's size, but would require us to know how large those are on the
///   target platform. This isn't terribly difficult - we could emit a const
///   expression that could call `core::mem::size_of` in order to determine the
///   size and check against the number of enum variants, but a) this would be
///   platform-specific and, b) even on Rust's smallest bit width platform (32),
///   this would require ~4 billion enum variants, which obviously isn't a thing.
/// - All fields of all variants are `FromBytes`.
fn derive_from_bytes_enum(
    ast: &DeriveInput,
    enm: &DataEnum,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = EnumRepr::from_attrs(&ast.attrs)?;
    let variants_required = 1usize << enum_size_from_repr(&repr)?;
    if enm.variants.len() != variants_required {
        return Err(
            Error::new_spanned(
                ast,
                format!(
                    "FromBytes only supported on {} enum with {} variants", repr
                    .repr_type_name(), variants_required
                ),
            ),
        );
    }
    Ok(
        ImplBlockBuilder::new(
                ast,
                enm,
                Trait::FromBytes,
                FieldBounds::ALL_SELF,
                zerocopy_crate,
            )
            .build(),
    )
}
