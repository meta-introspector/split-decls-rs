macro_rules! LayoutConstrainedPlaceVisitor {
    () => {
        struct LayoutConstrainedPlaceVisitor < 'a , 'tcx > { found : bool , thir : & 'a Thir < 'tcx > , tcx : TyCtxt < 'tcx > , }
    };
}

LayoutConstrainedPlaceVisitor!()