macro_rules! deps {
    () => {
        PlaceCollector!();
        TrackElem!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for PlaceCollector < '_ , 'tcx > { # [tracing :: instrument (level = "trace" , skip (self))] fn visit_place (& mut self , place : & Place < 'tcx > , ctxt : PlaceContext , _ : Location) { if ! ctxt . is_use () { return ; } self . register_place (* place) ; } fn visit_assign (& mut self , lhs : & Place < 'tcx > , rhs : & Rvalue < 'tcx > , location : Location) { self . super_assign (lhs , rhs , location) ; match rhs { Rvalue :: Use (Operand :: Move (rhs) | Operand :: Copy (rhs)) | Rvalue :: CopyForDeref (rhs) => { let Some (lhs) = self . register_place (* lhs) else { return } ; let Some (rhs) = self . register_place (* rhs) else { return } ; self . assignments . insert ((lhs , rhs)) ; } Rvalue :: Aggregate (kind , fields) => { let Some (mut lhs) = self . register_place (* lhs) else { return } ; match * * kind { AggregateKind :: Adt (_ , _ , _ , _ , Some (_)) => return , AggregateKind :: Adt (_ , variant , _ , _ , None) => { let ty = self . map . places [lhs] . ty ; if ty . is_enum () { lhs = self . map . register_place (ty , lhs , TrackElem :: Variant (variant)) ; } } AggregateKind :: RawPtr (..) | AggregateKind :: Array (_) | AggregateKind :: Tuple | AggregateKind :: Closure (..) | AggregateKind :: Coroutine (..) | AggregateKind :: CoroutineClosure (..) => { } } for (index , field) in fields . iter_enumerated () { if let Some (rhs) = field . place () && let Some (rhs) = self . register_place (rhs) { let lhs = self . map . register_place (self . map . places [rhs] . ty , lhs , TrackElem :: Field (index) ,) ; self . assignments . insert ((lhs , rhs)) ; } } } _ => { } } } }
    };
}

impl_255!()