macro_rules! deps {
    () => {
        YieldResumeEffect!();
        DefUse!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for YieldResumeEffect < '_ > { fn visit_place (& mut self , place : & mir :: Place < 'tcx > , context : PlaceContext , location : Location) { DefUse :: apply (self . 0 , * place , context) ; self . visit_projection (place . as_ref () , context , location) ; } fn visit_local (& mut self , local : Local , context : PlaceContext , _ : Location) { DefUse :: apply (self . 0 , local . into () , context) ; } }
    };
}

impl_157!();