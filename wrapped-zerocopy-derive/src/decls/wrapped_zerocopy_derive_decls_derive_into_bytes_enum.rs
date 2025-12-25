use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// If the type is an enum:
/// - It must have a defined representation (`repr`s `C`, `u8`, `u16`, `u32`,
///   `u64`, `usize`, `i8`, `i16`, `i32`, `i64`, or `isize`).
/// - It must have no padding bytes.
/// - Its fields must be `IntoBytes`.
fn derive_into_bytes_enum(
    ast: &DeriveInput,
    enm: &DataEnum,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let repr = EnumRepr::from_attrs(&ast.attrs)?;
    if !repr.is_c() && !repr.is_primitive() {
        return Err(
            Error::new(
                Span::call_site(),
                "must have #[repr(C)] or #[repr(Int)] attribute in order to guarantee this type's memory layout",
            ),
        );
    }
    let tag_type_definition = r#enum::generate_tag_enum(&repr, enm);
    Ok(
        ImplBlockBuilder::new(
                ast,
                enm,
                Trait::IntoBytes,
                FieldBounds::ALL_SELF,
                zerocopy_crate,
            )
            .padding_check(PaddingCheck::Enum {
                tag_type_definition,
            })
            .build(),
    )
}
