macro_rules! UnalignedPackedRef {
    () => {
        # [derive (Diagnostic)] # [diag (mir_transform_unaligned_packed_ref , code = E0793)] # [note] # [note (mir_transform_note_ub)] # [help] pub (crate) struct UnalignedPackedRef { # [primary_span] pub span : Span , }
    };
}

UnalignedPackedRef!()