use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn derive_into_bytes_struct(
    ast: &DeriveInput,
    strct: &DataStruct,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = StructUnionRepr::from_attrs(&ast.attrs)?;
    let is_transparent = repr.is_transparent();
    let is_c = repr.is_c();
    let is_packed_1 = repr.is_packed_1();
    let num_fields = strct.fields().len();
    let (padding_check, require_unaligned_fields) = if is_transparent || is_packed_1 {
        (None, false)
    } else if is_c && !repr.is_align_gt_1() && num_fields <= 1 {
        (None, false)
    } else if ast.generics.params.is_empty() {
        let is_syntactic_dst = strct
            .fields()
            .last()
            .map(|(_, _, ty)| matches!(ty, Type::Slice(_)))
            .unwrap_or(false);
        if is_c && is_syntactic_dst {
            (Some(PaddingCheck::ReprCStruct), false)
        } else {
            (Some(PaddingCheck::Struct), false)
        }
    } else if is_c && !repr.is_align_gt_1() {
        (None, true)
    } else {
        return Err(
            Error::new(
                Span::call_site(),
                "must have a non-align #[repr(...)] attribute in order to guarantee this type's memory layout",
            ),
        );
    };
    let field_bounds = if require_unaligned_fields {
        FieldBounds::All(&[TraitBound::Slf, TraitBound::Other(Trait::Unaligned)])
    } else {
        FieldBounds::ALL_SELF
    };
    Ok(
        ImplBlockBuilder::new(ast, strct, Trait::IntoBytes, field_bounds, zerocopy_crate)
            .padding_check(padding_check)
            .build(),
    )
}
