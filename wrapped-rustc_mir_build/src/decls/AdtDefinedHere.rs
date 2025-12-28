macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! AdtDefinedHere {
    () => {
        deps!();
        pub (crate) struct AdtDefinedHere < 'tcx > { pub (crate) adt_def_span : Span , pub (crate) ty : Ty < 'tcx > , pub (crate) variants : Vec < Variant > , }
    };
}

AdtDefinedHere!();