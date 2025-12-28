macro_rules! deps {
    () => {
        TraitRef!();
        InternalCx!();
        TraitRefHelpers!();
        DefId!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl < 'tcx , T : InternalCx < 'tcx > > TraitRefHelpers < 'tcx > for T { fn new_from_args (& self , trait_def_id : rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: TraitRef < 'tcx > { ty :: TraitRef :: new_from_args (self . tcx () , trait_def_id , args) } }
    };
}

impl_243!();