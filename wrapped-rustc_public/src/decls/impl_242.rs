macro_rules! deps {
    () => {
        ExistentialTraitRef!();
        InternalCx!();
        ExistentialTraitRefHelpers!();
        DefId!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl < 'tcx , T : InternalCx < 'tcx > > ExistentialTraitRefHelpers < 'tcx > for T { fn new_from_args (& self , trait_def_id : rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: ExistentialTraitRef < 'tcx > { ty :: ExistentialTraitRef :: new_from_args (self . tcx () , trait_def_id , args) } }
    };
}

impl_242!()