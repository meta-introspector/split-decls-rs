macro_rules! deps {
    () => {
        PatCx!();
        DeconstructedPat!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [doc = " Delegate to `uid`."] impl < Cx : PatCx > std :: hash :: Hash for DeconstructedPat < Cx > { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . uid . hash (state) ; } }
    };
}

impl_49!()