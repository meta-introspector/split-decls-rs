macro_rules! deps {
    () => {
        NoDrop!();
    };
}

macro_rules! IterTrait {
    () => {
        deps!();
        trait IterTrait < 'a , T : 'a > : Iterator < Item = & 'a T > + DoubleEndedIterator + ExactSizeIterator { fn clone_box (& self) -> Box < NoDrop < dyn IterTrait < 'a , T > + 'a > > ; }
    };
}

IterTrait!();