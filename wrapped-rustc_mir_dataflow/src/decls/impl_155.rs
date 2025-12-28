macro_rules! deps {
    () => {
        TransferFunction!();
        DefUse!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for TransferFunction < '_ > { fn visit_place (& mut self , place : & mir :: Place < 'tcx > , context : PlaceContext , location : Location) { if let PlaceContext :: MutatingUse (MutatingUseContext :: Yield) = context { return ; } match DefUse :: for_place (* place , context) { DefUse :: Def => { if let PlaceContext :: MutatingUse (MutatingUseContext :: Call | MutatingUseContext :: AsmOutput ,) = context { } else { self . 0 . kill (place . local) ; } } DefUse :: Use => self . 0 . gen_ (place . local) , DefUse :: PartialWrite | DefUse :: NonUse => { } } self . visit_projection (place . as_ref () , context , location) ; } fn visit_local (& mut self , local : Local , context : PlaceContext , _ : Location) { DefUse :: apply (self . 0 , local . into () , context) ; } }
    };
}

impl_155!();