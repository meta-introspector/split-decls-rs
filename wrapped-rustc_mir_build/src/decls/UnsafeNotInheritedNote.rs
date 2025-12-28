macro_rules! UnsafeNotInheritedNote {
    () => {
        # [derive (Subdiagnostic)] # [label (mir_build_unsafe_not_inherited)] pub (crate) struct UnsafeNotInheritedNote { # [primary_span] pub (crate) span : Span , }
    };
}

UnsafeNotInheritedNote!();