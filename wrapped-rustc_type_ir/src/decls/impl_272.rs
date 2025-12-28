macro_rules! deps {
    () => {
        CanonicalVarValues!();
        Interner!();
        GenericArgs!();
        GenericArg!();
        SliceLike!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < 'a , I : Interner > IntoIterator for & 'a CanonicalVarValues < I > { type Item = I :: GenericArg ; type IntoIter = < I :: GenericArgs as SliceLike > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . var_values . iter () } }
    };
}

impl_272!()