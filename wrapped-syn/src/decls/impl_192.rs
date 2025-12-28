macro_rules! deps {
    () => {
        Error!();
        IntoIter!();
        Iter!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Error { type Item = Error ; type IntoIter = Iter < 'a > ; fn into_iter (self) -> Self :: IntoIter { Iter { messages : self . messages . iter () , } } }
    };
}

impl_192!();