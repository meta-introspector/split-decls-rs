macro_rules! deps {
    () => {
        Interner!();
    };
}

macro_rules! IterInstantiated {
    () => {
        deps!();
        pub struct IterInstantiated < I : Interner , Iter : IntoIterator , A > { it : Iter :: IntoIter , cx : I , args : A , }
    };
}

IterInstantiated!();