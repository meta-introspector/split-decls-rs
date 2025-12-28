macro_rules! is_sized_raw {
    () => {
        fn is_sized_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Sized) }
    };
}

is_sized_raw!();