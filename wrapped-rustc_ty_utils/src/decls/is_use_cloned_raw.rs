macro_rules! is_use_cloned_raw {
    () => {
        fn is_use_cloned_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { is_item_raw (tcx , query , LangItem :: UseCloned) }
    };
}

is_use_cloned_raw!()