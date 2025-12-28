macro_rules! TyCtxt {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct TyCtxt < 'tcx > (pub PhantomData < & 'tcx () >) ;
    };
}

TyCtxt!()