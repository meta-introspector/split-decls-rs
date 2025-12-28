macro_rules! is_copy_raw {
    () => {
        fn is_copy_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Copy) }
    };
}

is_copy_raw!()