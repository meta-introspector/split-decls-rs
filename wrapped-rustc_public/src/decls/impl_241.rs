macro_rules! deps {
    () => {
        ExistentialProjection!();
        InternalCx!();
        DefId!();
        ExistentialProjectionHelpers!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < 'tcx , T : InternalCx < 'tcx > > ExistentialProjectionHelpers < 'tcx > for T { fn new_from_args (& self , def_id : rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > , term : ty :: Term < 'tcx > ,) -> ty :: ExistentialProjection < 'tcx > { ty :: ExistentialProjection :: new_from_args (self . tcx () , def_id , args , term) } }
    };
}

impl_241!()