macro_rules! deps {
    () => {
        WitnessPat!();
        PatCx!();
        PlaceCtxt!();
        Constructor!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'a , Cx : PatCx > PlaceCtxt < 'a , Cx > { fn ctor_arity (& self , ctor : & Constructor < Cx >) -> usize { self . cx . ctor_arity (ctor , self . ty) } fn wild_from_ctor (& self , ctor : Constructor < Cx >) -> WitnessPat < Cx > { WitnessPat :: wild_from_ctor (self . cx , ctor , self . ty . clone ()) } }
    };
}

impl_96!()