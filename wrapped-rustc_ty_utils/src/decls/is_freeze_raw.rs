macro_rules! is_freeze_raw {
    () => {
        fn is_freeze_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Freeze) }
    };
}

is_freeze_raw!()