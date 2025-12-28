macro_rules! min {
    () => {
        fn min (vis1 : ty :: Visibility , vis2 : ty :: Visibility , tcx : TyCtxt < '_ >) -> ty :: Visibility { if vis1 . is_at_least (vis2 , tcx) { vis2 } else { vis1 } }
    };
}

min!()