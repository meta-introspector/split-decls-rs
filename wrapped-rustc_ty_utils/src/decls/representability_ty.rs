macro_rules! representability_ty {
    () => {
        fn representability_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Representability { match * ty . kind () { ty :: Adt (..) => tcx . representability_adt_ty (ty) , ty :: Array (ty , _) => representability_ty (tcx , ty) , ty :: Tuple (tys) => { for ty in tys { rtry ! (representability_ty (tcx , ty)) ; } Representability :: Representable } _ => Representability :: Representable , } }
    };
}

representability_ty!()