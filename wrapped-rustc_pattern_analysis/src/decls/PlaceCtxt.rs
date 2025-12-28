macro_rules! deps {
    () => {
        PatCx!();
    };
}

macro_rules! PlaceCtxt {
    () => {
        deps!();
        # [doc = " Context that provides information local to a place under investigation."] struct PlaceCtxt < 'a , Cx : PatCx > { cx : & 'a Cx , # [doc = " Type of the place under investigation."] ty : & 'a Cx :: Ty , }
    };
}

PlaceCtxt!()