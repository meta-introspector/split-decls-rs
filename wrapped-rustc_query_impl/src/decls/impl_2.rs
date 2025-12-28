macro_rules! deps {
    () => {
        QueryCtxt!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 'tcx > std :: ops :: Deref for QueryCtxt < 'tcx > { type Target = TyCtxt < 'tcx > ; # [inline] fn deref (& self) -> & Self :: Target { & self . tcx } }
    };
}

impl_2!();