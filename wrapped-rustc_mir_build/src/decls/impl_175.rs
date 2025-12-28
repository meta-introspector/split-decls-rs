macro_rules! deps {
    () => {
        LayoutConstrainedPlaceVisitor!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < 'a , 'tcx > LayoutConstrainedPlaceVisitor < 'a , 'tcx > { fn new (thir : & 'a Thir < 'tcx > , tcx : TyCtxt < 'tcx >) -> Self { Self { found : false , thir , tcx } } }
    };
}

impl_175!();