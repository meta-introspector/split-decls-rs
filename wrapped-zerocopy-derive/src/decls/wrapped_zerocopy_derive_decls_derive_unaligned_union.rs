use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Like structs, a union is `Unaligned` if:
/// - `repr(align)` is no more than 1 and either
///   - `repr(C)` or `repr(transparent)` and
///     - all fields `Unaligned`
///   - `repr(packed)`
fn derive_unaligned_union(
    ast: &DeriveInput,
    unn: &DataUnion,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = StructUnionRepr::from_attrs(&ast.attrs)?;
    repr.unaligned_validate_no_align_gt_1()?;
    let field_type_trait_bounds = if repr.is_packed_1() {
        FieldBounds::None
    } else if repr.is_c() || repr.is_transparent() {
        FieldBounds::ALL_SELF
    } else {
        return Err(
            Error::new(
                Span::call_site(),
                "must have #[repr(C)], #[repr(transparent)], or #[repr(packed)] attribute in order to guarantee this type's alignment",
            ),
        );
    };
    Ok(ImplBlockBuilder::new(
        ast,
        unn,
        Trait::Unaligned,
        field_type_trait_bounds,
        zerocopy_crate,
    )
    .build())
}
