macro_rules! deps {
    () => {
        Error!();
        IntoIter!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl IntoIterator for Error { type Item = Error ; type IntoIter = IntoIter ; fn into_iter (self) -> Self :: IntoIter { IntoIter { messages : self . messages . into_iter () , } } }
    };
}

impl_189!();