macro_rules! is_unpin_raw {
    () => {
        fn is_unpin_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Unpin) }
    };
}

is_unpin_raw!();