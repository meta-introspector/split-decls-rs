macro_rules! EnumVariantSameName {
    () => {
        # [derive (Subdiagnostic)] # [note (passes_enum_variant_same_name)] pub (crate) struct EnumVariantSameName < 'tcx > { # [primary_span] pub variant_span : Span , pub dead_name : Symbol , pub dead_descr : & 'tcx str , }
    };
}

EnumVariantSameName!();