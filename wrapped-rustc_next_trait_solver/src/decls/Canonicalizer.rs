macro_rules! deps {
    () => {
        SolverDelegate!();
        CanonicalizeMode!();
    };
}

macro_rules! Canonicalizer {
    () => {
        deps!();
        pub struct Canonicalizer < 'a , D : SolverDelegate < Interner = I > , I : Interner > { delegate : & 'a D , canonicalize_mode : CanonicalizeMode , variables : & 'a mut Vec < I :: GenericArg > , var_kinds : Vec < CanonicalVarKind < I > > , variable_lookup_table : HashMap < I :: GenericArg , usize > , # [doc = " Maps each `sub_unification_table_root_var` to the index of the first"] # [doc = " variable which used it."] # [doc = ""] # [doc = " This means in case two type variables have the same sub relations root,"] # [doc = " we set the `sub_root` of the second variable to the position of the first."] # [doc = " Otherwise the `sub_root` of each type variable is just its own position."] sub_root_lookup_table : HashMap < ty :: TyVid , usize > , binder_index : ty :: DebruijnIndex , # [doc = " We only use the debruijn index during lookup. We don't need to"] # [doc = " track the `variables` as each generic arg only results in a single"] # [doc = " bound variable regardless of how many times it is encountered."] cache : HashMap < (ty :: DebruijnIndex , I :: Ty) , I :: Ty > , }
    };
}

Canonicalizer!();