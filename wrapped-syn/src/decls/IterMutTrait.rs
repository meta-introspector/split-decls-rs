macro_rules! IterMutTrait {
    () => {
        trait IterMutTrait < 'a , T : 'a > : DoubleEndedIterator < Item = & 'a mut T > + ExactSizeIterator < Item = & 'a mut T > { }
    };
}

IterMutTrait!()