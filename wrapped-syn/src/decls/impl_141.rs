macro_rules! deps {
    () => {
        IntoIter!();
        Punctuated!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl IntoIterator for Fields { type Item = Field ; type IntoIter = punctuated :: IntoIter < Field > ; fn into_iter (self) -> Self :: IntoIter { match self { Fields :: Unit => Punctuated :: < Field , () > :: new () . into_iter () , Fields :: Named (f) => f . named . into_iter () , Fields :: Unnamed (f) => f . unnamed . into_iter () , } } }
    };
}

impl_141!();