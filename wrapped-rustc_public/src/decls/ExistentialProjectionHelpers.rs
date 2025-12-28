macro_rules! deps {
    () => {
        ExistentialProjection!();
        DefId!();
    };
}

macro_rules! ExistentialProjectionHelpers {
    () => {
        deps!();
        pub (crate) trait ExistentialProjectionHelpers < 'tcx > { fn new_from_args (& self , def_id : rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > , term : ty :: Term < 'tcx > ,) -> ty :: ExistentialProjection < 'tcx > ; }
    };
}

ExistentialProjectionHelpers!()