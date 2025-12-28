macro_rules! deps {
    () => {
        DefId!();
        ExistentialTraitRef!();
    };
}

macro_rules! ExistentialTraitRefHelpers {
    () => {
        deps!();
        pub (crate) trait ExistentialTraitRefHelpers < 'tcx > { fn new_from_args (& self , trait_def_id : rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: ExistentialTraitRef < 'tcx > ; }
    };
}

ExistentialTraitRefHelpers!()