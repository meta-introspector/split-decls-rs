macro_rules! deps {
    () => {
        NoDrop!();
        IterTrait!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator over borrowed values of type `&T`."] # [doc = ""] # [doc = " Refer to the [module documentation] for details about punctuated sequences."] # [doc = ""] # [doc = " [module documentation]: self"] pub struct Iter < 'a , T : 'a > { inner : Box < NoDrop < dyn IterTrait < 'a , T > + 'a > > , }
    };
}

Iter!();