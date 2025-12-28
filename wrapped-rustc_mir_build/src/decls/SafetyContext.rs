macro_rules! deps {
    () => {
        NestedUsedBlock!();
    };
}

macro_rules! SafetyContext {
    () => {
        deps!();
        # [derive (Clone)] enum SafetyContext { Safe , BuiltinUnsafeBlock , UnsafeFn , UnsafeBlock { span : Span , hir_id : HirId , used : bool , nested_used_blocks : Vec < NestedUsedBlock > } , }
    };
}

SafetyContext!()