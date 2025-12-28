macro_rules! deps {
    () => {
        InternalCx!();
        Binder!();
        BridgeTys!();
        RustcInternal!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        # [allow (rustc :: usage_of_qualified_ty)] impl < T > RustcInternal for Binder < T > where T : RustcInternal , for < 'tcx > T :: T < 'tcx > : rustc_ty :: TypeVisitable < rustc_ty :: TyCtxt < 'tcx > > , { type T < 'tcx > = rustc_ty :: Binder < 'tcx , T :: T < 'tcx > > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { rustc_ty :: Binder :: bind_with_vars (self . value . internal (tables , tcx) , tcx . mk_bound_variable_kinds_from_iter (self . bound_vars . iter () . map (| bound | bound . internal (tables , tcx)) ,) ,) } }
    };
}

impl_75!();