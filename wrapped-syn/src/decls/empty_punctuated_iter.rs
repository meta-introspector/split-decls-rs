macro_rules! deps {
    () => {
        Iter!();
        NoDrop!();
    };
}

macro_rules! empty_punctuated_iter {
    () => {
        deps!();
        # [cfg (any (feature = "full" , feature = "derive"))] pub (crate) fn empty_punctuated_iter < 'a , T > () -> Iter < 'a , T > { Iter { inner : Box :: new (NoDrop :: new (iter :: empty ())) , } }
    };
}

empty_punctuated_iter!();