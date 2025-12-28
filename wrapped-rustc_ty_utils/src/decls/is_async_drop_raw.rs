macro_rules! is_async_drop_raw {
    () => {
        fn is_async_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { is_item_raw (tcx , query , LangItem :: AsyncDrop) }
    };
}

is_async_drop_raw!();