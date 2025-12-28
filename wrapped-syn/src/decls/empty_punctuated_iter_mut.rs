macro_rules! deps {
    () => {
        NoDrop!();
        IterMut!();
    };
}

macro_rules! empty_punctuated_iter_mut {
    () => {
        deps!();
        # [cfg (any (feature = "full" , feature = "derive"))] pub (crate) fn empty_punctuated_iter_mut < 'a , T > () -> IterMut < 'a , T > { IterMut { inner : Box :: new (NoDrop :: new (iter :: empty ())) , } }
    };
}

empty_punctuated_iter_mut!();