macro_rules! deps {
    () => {
        AdtDefinedHere!();
        Variant!();
    };
}

macro_rules! report_adt_defined_here {
    () => {
        deps!();
        fn report_adt_defined_here < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , witnesses : & [WitnessPat < '_ , 'tcx >] , point_at_non_local_ty : bool ,) -> Option < AdtDefinedHere < 'tcx > > { let ty = ty . peel_refs () ; let ty :: Adt (def , _) = ty . kind () else { return None ; } ; let adt_def_span = tcx . hir_get_if_local (def . did ()) . and_then (| node | node . ident ()) . map (| ident | ident . span) ; let adt_def_span = if point_at_non_local_ty { adt_def_span . unwrap_or_else (| | tcx . def_span (def . did ())) } else { adt_def_span ? } ; let mut variants = vec ! [] ; for span in maybe_point_at_variant (tcx , * def , witnesses . iter () . take (5)) { variants . push (Variant { span }) ; } Some (AdtDefinedHere { adt_def_span , ty , variants }) }
    };
}

report_adt_defined_here!()