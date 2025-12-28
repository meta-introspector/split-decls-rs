macro_rules! params_in_repr {
    () => {
        fn params_in_repr (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> DenseBitSet < u32 > { let adt_def = tcx . adt_def (def_id) ; let generics = tcx . generics_of (def_id) ; let mut params_in_repr = DenseBitSet :: new_empty (generics . own_params . len ()) ; for variant in adt_def . variants () { for field in variant . fields . iter () { params_in_repr_ty (tcx , tcx . type_of (field . did) . instantiate_identity () , & mut params_in_repr ,) ; } } params_in_repr }
    };
}

params_in_repr!()