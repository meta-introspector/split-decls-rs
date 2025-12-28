macro_rules! deps {
    () => {
        Iter!();
        IntoIter!();
    };
}

macro_rules! PrivateIter {
    () => {
        deps!();
        struct PrivateIter < 'a , T : 'a , P : 'a > { inner : slice :: Iter < 'a , (T , P) > , last : option :: IntoIter < & 'a T > , }
    };
}

PrivateIter!();