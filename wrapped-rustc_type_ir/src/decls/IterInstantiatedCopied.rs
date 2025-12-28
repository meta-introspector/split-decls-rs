macro_rules! deps {
    () => {
        Interner!();
        GenericArg!();
    };
}

macro_rules! IterInstantiatedCopied {
    () => {
        deps!();
        pub struct IterInstantiatedCopied < 'a , I : Interner , Iter : IntoIterator > { it : Iter :: IntoIter , cx : I , args : & 'a [I :: GenericArg] , }
    };
}

IterInstantiatedCopied!();