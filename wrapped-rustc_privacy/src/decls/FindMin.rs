macro_rules! deps {
    () => {
        VisibilityLike!();
    };
}

macro_rules! FindMin {
    () => {
        deps!();
        # [doc = " Visitor used to determine impl visibility and reachability."] struct FindMin < 'a , 'tcx , VL : VisibilityLike , const SHALLOW : bool > { tcx : TyCtxt < 'tcx > , effective_visibilities : & 'a EffectiveVisibilities , min : VL , }
    };
}

FindMin!()