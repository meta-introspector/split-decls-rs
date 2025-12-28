macro_rules! deps {
    () => {
        WitnessPat!();
        PatCx!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [doc = " This is best effort and not good enough for a `Display` impl."] impl < Cx : PatCx > fmt :: Debug for WitnessPat < Cx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . ctor () . fmt_fields (f , self . ty () , self . fields . iter ()) } }
    };
}

impl_58!()