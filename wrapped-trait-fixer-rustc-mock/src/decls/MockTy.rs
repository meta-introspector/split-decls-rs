macro_rules! MockTy {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct MockTy < 'tcx > (PhantomData < & 'tcx () >) ;
    };
}

MockTy!()