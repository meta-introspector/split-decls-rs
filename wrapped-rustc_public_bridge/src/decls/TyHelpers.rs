macro_rules! TyHelpers {
    () => {
        pub trait TyHelpers < 'tcx > { fn new_foreign (& self , def_id : DefId) -> Ty < 'tcx > ; }
    };
}

TyHelpers!()