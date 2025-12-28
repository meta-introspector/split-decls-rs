macro_rules! deps {
    () => {
        IterMut!();
        IntoIter!();
    };
}

macro_rules! PrivateIterMut {
    () => {
        deps!();
        struct PrivateIterMut < 'a , T : 'a , P : 'a > { inner : slice :: IterMut < 'a , (T , P) > , last : option :: IntoIter < & 'a mut T > , }
    };
}

PrivateIterMut!();