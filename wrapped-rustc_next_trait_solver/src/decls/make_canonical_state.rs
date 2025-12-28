macro_rules! deps {
    () => {
        Canonicalizer!();
        SolverDelegate!();
    };
}

macro_rules! make_canonical_state {
    () => {
        deps!();
        # [doc = " Used by proof trees to be able to recompute intermediate actions while"] # [doc = " evaluating a goal. The `var_values` not only include the bound variables"] # [doc = " of the query input, but also contain all unconstrained inference vars"] # [doc = " created while evaluating this goal."] pub (in crate :: solve) fn make_canonical_state < D , T , I > (delegate : & D , var_values : & [I :: GenericArg] , max_input_universe : ty :: UniverseIndex , data : T ,) -> inspect :: CanonicalState < I , T > where D : SolverDelegate < Interner = I > , I : Interner , T : TypeFoldable < I > , { let var_values = CanonicalVarValues { var_values : delegate . cx () . mk_args (var_values) } ; let state = inspect :: State { var_values , data } ; let state = eager_resolve_vars (delegate , state) ; Canonicalizer :: canonicalize_response (delegate , max_input_universe , & mut vec ! [] , state) }
    };
}

make_canonical_state!();