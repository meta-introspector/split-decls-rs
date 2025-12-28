macro_rules! representability_adt_ty {
    () => {
        fn representability_adt_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Representability { let ty :: Adt (adt , args) = ty . kind () else { bug ! ("expected adt") } ; if let Some (def_id) = adt . did () . as_local () { rtry ! (tcx . representability (def_id)) ; } let params_in_repr = tcx . params_in_repr (adt . did ()) ; for (i , arg) in args . iter () . enumerate () { if let ty :: GenericArgKind :: Type (ty) = arg . kind () { if params_in_repr . contains (i as u32) { rtry ! (representability_ty (tcx , ty)) ; } } } Representability :: Representable }
    };
}

representability_adt_ty!()