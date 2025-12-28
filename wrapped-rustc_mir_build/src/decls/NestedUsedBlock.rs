macro_rules! NestedUsedBlock {
    () => {
        # [derive (Clone , Copy)] struct NestedUsedBlock { hir_id : HirId , span : Span , }
    };
}

NestedUsedBlock!()