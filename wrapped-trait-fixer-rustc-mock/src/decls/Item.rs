macro_rules! Item {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Item < 'tcx > (PhantomData < & 'tcx () >) ;
    };
}

Item!()