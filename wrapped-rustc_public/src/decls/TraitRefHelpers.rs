macro_rules! deps {
    () => {
        DefId!();
        TraitRef!();
    };
}

macro_rules! TraitRefHelpers {
    () => {
        deps!();
        pub (crate) trait TraitRefHelpers < 'tcx > { fn new_from_args (& self , trait_def_id : rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: TraitRef < 'tcx > ; }
    };
}

TraitRefHelpers!()