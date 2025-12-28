macro_rules! deps {
    () => {
        PatCx!();
        PlaceCtxt!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < 'a , Cx : PatCx > fmt :: Debug for PlaceCtxt < 'a , Cx > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("PlaceCtxt") . field ("ty" , self . ty) . finish () } }
    };
}

impl_95!()