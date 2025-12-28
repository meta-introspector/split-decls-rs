macro_rules! TestReachabilityVisitor {
    () => {
        # [doc = " Visitor, used for EffectiveVisibilities table checking"] pub struct TestReachabilityVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , effective_visibilities : & 'a EffectiveVisibilities , }
    };
}

TestReachabilityVisitor!();