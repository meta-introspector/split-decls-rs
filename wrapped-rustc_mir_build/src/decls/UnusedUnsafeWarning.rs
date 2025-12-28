macro_rules! deps {
    () => {
        UnusedUnsafeEnclosing!();
    };
}

macro_rules! UnusedUnsafeWarning {
    () => {
        deps!();
        struct UnusedUnsafeWarning { hir_id : HirId , block_span : Span , enclosing_unsafe : Option < UnusedUnsafeEnclosing > , }
    };
}

UnusedUnsafeWarning!()