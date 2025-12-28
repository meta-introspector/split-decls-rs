macro_rules! deps {
    () => {
        IterMutTrait!();
    };
}

macro_rules! impl_660 {
    () => {
        deps!();
        impl < 'a , T , I > IterMutTrait < 'a , T > for I where T : 'a , I : DoubleEndedIterator < Item = & 'a mut T > + ExactSizeIterator < Item = & 'a mut T > + 'a , { }
    };
}

impl_660!()