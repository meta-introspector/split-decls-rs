macro_rules! deps {
    () => {
        TrivialDrop!();
        IterTrait!();
        NoDrop!();
    };
}

macro_rules! impl_648 {
    () => {
        deps!();
        impl < 'a , T , I > IterTrait < 'a , T > for I where T : 'a , I : DoubleEndedIterator < Item = & 'a T > + ExactSizeIterator < Item = & 'a T > + Clone + TrivialDrop + 'a , { fn clone_box (& self) -> Box < NoDrop < dyn IterTrait < 'a , T > + 'a > > { Box :: new (NoDrop :: new (self . clone ())) } }
    };
}

impl_648!();