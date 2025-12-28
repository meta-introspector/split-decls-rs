macro_rules! deps {
    () => {
        PatCx!();
        PlaceCtxt!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'a , Cx : PatCx > Copy for PlaceCtxt < 'a , Cx > { }
    };
}

impl_93!();