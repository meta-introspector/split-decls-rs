macro_rules! deps {
    () => {
        VarianceDiagInfo!();
        TypeRelation!();
        RelateResult!();
        Interner!();
        GenericArgs!();
    };
}

macro_rules! relate_args_invariantly {
    () => {
        deps!();
        # [inline] pub fn relate_args_invariantly < I : Interner , R : TypeRelation < I > > (relation : & mut R , a_arg : I :: GenericArgs , b_arg : I :: GenericArgs ,) -> RelateResult < I , I :: GenericArgs > { relation . cx () . mk_args_from_iter (iter :: zip (a_arg . iter () , b_arg . iter ()) . map (| (a , b) | { relation . relate_with_variance (ty :: Invariant , VarianceDiagInfo :: default () , a , b) })) }
    };
}

relate_args_invariantly!();