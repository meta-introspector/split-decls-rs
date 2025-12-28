macro_rules! deps {
    () => {
        SerdeStructVisitor!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < S : SerializeStruct > SerdeStructVisitor < S > { # [doc = " Completes serializing the visited object, returning `Ok(())` if all"] # [doc = " fields were serialized correctly, or `Error(S::Error)` if a field could"] # [doc = " not be serialized."] pub fn finish (self) -> Result < S :: Ok , S :: Error > { self . state ? ; self . serializer . end () } }
    };
}

impl_30!()