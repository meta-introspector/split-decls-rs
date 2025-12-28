macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! maybe_point_at_variant {
    () => {
        deps!();
        fn maybe_point_at_variant < 'a , 'p : 'a , 'tcx : 'p > (tcx : TyCtxt < 'tcx > , def : AdtDef < 'tcx > , patterns : impl Iterator < Item = & 'a WitnessPat < 'p , 'tcx > > ,) -> Vec < Span > { let mut covered = vec ! [] ; for pattern in patterns { if let Constructor :: Variant (variant_index) = pattern . ctor () { if let ty :: Adt (this_def , _) = pattern . ty () . kind () && this_def . did () != def . did () { continue ; } let sp = def . variant (* variant_index) . ident (tcx) . span ; if covered . contains (& sp) { continue ; } covered . push (sp) ; } covered . extend (maybe_point_at_variant (tcx , def , pattern . iter_fields ())) ; } covered }
    };
}

maybe_point_at_variant!();