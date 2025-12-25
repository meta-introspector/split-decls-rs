use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enum is `Unaligned` if:
/// - No `repr(align(N > 1))`
/// - `repr(u8)` or `repr(i8)`
fn derive_unaligned_enum(
    ast: &DeriveInput,
    enm: &DataEnum,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = EnumRepr::from_attrs(&ast.attrs)?;
    repr.unaligned_validate_no_align_gt_1()?;
    if !repr.is_u8() && !repr.is_i8() {
        return Err(
            Error::new(
                Span::call_site(),
                "must have #[repr(u8)] or #[repr(i8)] attribute in order to guarantee this type's alignment",
            ),
        );
    }
    Ok(ImplBlockBuilder::new(
        ast,
        enm,
        Trait::Unaligned,
        FieldBounds::ALL_SELF,
        zerocopy_crate,
    )
    .build())
}
