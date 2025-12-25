use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A struct is `TryFromBytes` if:
/// - all fields are `TryFromBytes`
fn derive_try_from_bytes_struct(
    ast: &DeriveInput,
    strct: &DataStruct,
    top_level: Trait,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let extras = try_gen_trivial_is_bit_valid(ast, top_level, zerocopy_crate)
        .unwrap_or_else(|| {
            let fields = strct.fields();
            let field_names = fields.iter().map(|(_vis, name, _ty)| name);
            let field_tys = fields.iter().map(|(_vis, _name, ty)| ty);
            quote!(
                fn is_bit_valid < ___ZerocopyAliasing > (mut candidate : #
                zerocopy_crate::Maybe < Self, ___ZerocopyAliasing >,) -> #
                zerocopy_crate::util::macro_util::core_reexport::primitive::bool where
                ___ZerocopyAliasing : # zerocopy_crate::pointer::invariant::Reference, {
                use # zerocopy_crate::util::macro_util::core_reexport; use #
                zerocopy_crate::pointer::PtrInner; true # (&& { let field_candidate =
                unsafe { let project = | slf : PtrInner <'_, Self >| { let slf = slf
                .as_non_null().as_ptr(); let field = core_reexport::ptr::addr_of_mut!((*
                slf).# field_names); let ptr = unsafe {
                core_reexport::ptr::NonNull::new_unchecked(field) }; unsafe {
                PtrInner::new(ptr) } }; candidate.reborrow()
                .cast_unsized_unchecked(project) }; <# field_tys as #
                zerocopy_crate::TryFromBytes >::is_bit_valid(field_candidate) }) * }
            )
        });
    Ok(
        ImplBlockBuilder::new(
                ast,
                strct,
                Trait::TryFromBytes,
                FieldBounds::ALL_SELF,
                zerocopy_crate,
            )
            .inner_extras(extras)
            .build(),
    )
}
