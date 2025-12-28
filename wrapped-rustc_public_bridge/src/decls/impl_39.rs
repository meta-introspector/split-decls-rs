macro_rules! deps {
    () => {
        TyHelpers!();
        Bridge!();
        CompilerCtxt!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > TyHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { fn new_foreign (& self , def_id : DefId) -> ty :: Ty < 'tcx > { ty :: Ty :: new_foreign (self . tcx , def_id) } }
    };
}

impl_39!()