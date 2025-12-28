macro_rules! ConstPatternDependsOnGenericParameter {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_const_pattern_depends_on_generic_parameter , code = E0158)] pub (crate) struct ConstPatternDependsOnGenericParameter { # [primary_span] # [label] pub (crate) span : Span , }
    };
}

ConstPatternDependsOnGenericParameter!();